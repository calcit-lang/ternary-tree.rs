# TernaryTreeList 策略优化与不可变数据结构性能对比

日期：2026-08-26

基线：`59f60ac`（修改前工作树）

平台：Apple M1 Pro（8 核、16 GB），macOS Darwin 25.5.0，arm64

工具链：`rustc 1.94.0`，Criterion 0.6.0

## 结论

本轮没有改变 `TernaryTreeList` 的公开 API、2–3 节点表示或 finger-tree 风格的浅—深—浅布局，重点是减少更新路径上的无效工作：

- 迭代吞吐提升约 **31.4%**。
- `index_of`、`last_index_of`、`find_index` 的命中/未命中场景提升约 **15.4%–25.6%**。
- `From<Vec<T>>` 现在直接移动元素；构造过程对每个输入元素的克隆次数从 2 次降到 **0 次**。
- `push_left` / `push_right` 不再逐层克隆正在插入的新节点。对 `usize` 基准影响不显著，但消除了随路径深度增长的 `T::clone()`，对 `String`、运行时值等非平凡类型更重要。
- 与线程安全的 `rpds` 持久化结构相比，ternary tree 更擅长批量构造、尾部持续追加和尾部持续删除；32 叉 persistent vector 更擅长随机访问和单点更新；persistent list 更擅长纯头部操作。

因此，这个结构的核心优势仍是：**用同一个不可变序列同时覆盖头尾操作、索引、切片与拼接**。如果工作负载几乎只有随机访问，应优先考虑高分支 persistent vector；如果只有头部栈操作，应优先考虑 persistent list。

## 优化内容

### 1. 更新路径使用所有权传递

原实现的 `push_right_side` / `push_left_side` 已经拥有待插入的 `TernaryTree<T>`，但递归时仍调用 `item.to_owned()`。这会在每一层复制叶值或增加子树引用计数。

修改后先缓存 `item.len()`，再把 `item` 直接移动到下一层。已有树仍通过 `Arc` 结构共享，持久化语义不变。

### 2. 拥有所有权的批量构造不再克隆元素

原来的 `From<Vec<T>>` 先借用 `Vec` 中的元素、克隆成叶节点，再由重建逻辑再次复制叶节点。新增的 value-oriented builder 直接消费 iterator，并构造与原算法相同的 finger-shaped 布局。

行为测试使用带克隆计数器的元素确认：

- `TernaryTreeList::from(Vec<T>)`：输入元素克隆数为 0；
- `push_right(new_item)`：新元素沿递归路径的克隆数为 0。

从 `&Vec<T>` 和数组引用构造仍必须克隆元素一次，这是借用输入的必然成本。

### 3. 迭代器只保存“待访问子树”

原迭代器在栈中保存 `(node, stage)`，一个二叉/三叉节点会被多次压栈和弹栈。新实现按反向顺序一次压入子节点，节点只处理一次，并维护 `remaining`：

- 减少分支判断和栈操作；
- 提供精确 `size_hint`；
- 实现 `ExactSizeIterator`。

### 4. 搜索改为无堆分配的短路递归

原搜索为每次查询分配 `Vec` 工作栈并携带 base offset。树高是对数级，因此使用调用栈更合适。新实现直接递归并在命中时短路，不克隆 `Arc` 或 predicate。

## 修改前后基准

数据规模为 10,000 个 `usize`；搜索基准每轮执行 1,000 次查询。下表使用 Criterion 报告的点估计/变化比例。

| 场景 | 修改前 | 修改后 | 变化 |
| --- | ---: | ---: | ---: |
| 完整迭代 | 50.304 µs | 34.758 µs | **-31.4%** |
| `index_of` 命中 | 17.541 ms | 14.854 ms | **-15.4%** |
| `last_index_of` 命中 | 17.644 ms | 13.549 ms | **-23.8%** |
| `index_of` 未命中 | 35.925 ms | 29.811 ms | **-15.6%** |
| `last_index_of` 未命中 | 34.760 ms | 26.381 ms | **-23.9%** |
| `find_index` 命中 | 22.070 ms | 16.641 ms | **-25.0%** |
| `find_index` 未命中 | 44.208 ms | 32.929 ms | **-25.6%** |
| 10,000 次 `push_right<usize>` | 1.162 ms | 1.168 ms | 无统计显著变化 |

`get`、`loop_get`、`ref_get`、递归 `traverse` 和 drop 路径没有策略性改动；短基准中的小幅波动不视为本轮收益或回退。

## 与其他不可变算法对比

对比对象为 `rpds 1.2.1`：

- `VectorSync<T>`：32 叉 bitmapped vector trie，线程安全结构共享；用于尾部操作、随机访问、单点更新和迭代对比。
- `ListSync<T>`：经典 persistent singly linked list，线程安全结构共享；用于头部操作和迭代对比。

这里使用 `Sync` 版本是为了与基于 `Arc` 的 `TernaryTreeList` 保持相近的原子引用计数成本。所有更新都调用返回新结构的不可变 API，而非 `_mut` API；仅基准准备阶段使用 mutable builder。

| 工作负载 | TernaryTreeList | 对比结构 | 对比时间 | 结果 |
| --- | ---: | --- | ---: | ---: |
| 从 `Vec`/iterator 批量构造 10k | 218.00 µs | `VectorSync` | 338.51 µs | ternary **1.55× 快** |
| persistent 尾部追加 10k | 1.179 ms | `VectorSync` | 3.819 ms | ternary **3.24× 快** |
| persistent 头部追加 10k | 1.241 ms | `ListSync` | 538.62 µs | ternary **2.30× 慢** |
| 1k 次随机位置更新 | 555.39 µs | `VectorSync` | 429.08 µs | ternary **1.29× 慢** |
| 10k 次随机读取 | 815.12 µs | `VectorSync` | 24.873 µs | ternary **32.8× 慢** |
| 迭代 10k | 56.444 µs | `VectorSync` | 39.715 µs | ternary **1.42× 慢** |
| 迭代 10k | 56.444 µs | `ListSync` | 15.836 µs | ternary **3.56× 慢** |
| persistent 尾部删除 10k | 2.063 ms | `VectorSync` | 2.952 ms | ternary **1.43× 快** |
| persistent 头部删除 10k | 2.054 ms | `ListSync` | 205.66 µs | ternary **9.99× 慢** |

这些结果不是“同一抽象的唯一排名”：`ListSync` 不提供高效随机访问和尾部更新，`VectorSync` 也不是双端结构。对比的意义是界定 ternary tree 作为通用不可变序列的性能边界。

## 如何复现

```bash
cargo test --all-targets --offline

cargo bench --bench immutable_comparison --offline -- \
  --noplot --warm-up-time 0.5 --measurement-time 1 --sample-size 20
```

修改前后回归基准使用：

```bash
# 修改前保存基线
cargo bench --bench iterator  -- --noplot --save-baseline before_strategy \
  --warm-up-time 0.5 --measurement-time 1 --sample-size 10
cargo bench --bench searching -- --noplot --save-baseline before_strategy \
  --warm-up-time 0.5 --measurement-time 1 --sample-size 10

# 修改后对比
cargo bench --bench iterator  -- --noplot --baseline before_strategy \
  --warm-up-time 0.5 --measurement-time 1 --sample-size 10
cargo bench --bench searching -- --noplot --baseline before_strategy \
  --warm-up-time 0.5 --measurement-time 1 --sample-size 10
```

## 后续优化方向

1. 随机访问是最明显短板。可实验“叶块（small leaf arrays）+ 更高内部扇出”，但这会改变现有节点表示和更新策略，应单独设计并评估内存占用。
2. 当前 iterator 仍使用 heap `Vec` 保存待访问节点。可比较 small-stack/inline stack，但需要检查小列表的额外对象尺寸与大树溢出路径。
3. `TernaryTreeList<T>` 的实现整体要求 `Display + Debug + Ord + Hash` 等宽泛 trait bound。拆分 impl bound 主要改善可用性和编译生成，不预计直接改善本轮运行时热点。
4. 对真实 Calcit value 类型补充含非平凡 `Clone`/`Drop` 成本的应用级 trace benchmark，比 `usize` 微基准更能体现所有权传递的收益。
