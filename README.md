## Persistent structural sharing tree for Calcit Runtime

> a variant of 2-3 tree, with enhancements on ternary branching, optimized with tricks like finger-tree.

`t.pop_left()` and `t.push_right(..)` is optimized to be amortized `O(1)` at best cases and `O(log n)` when restructuring involed.

Tree layout from 0 to 159 watch [video](https://www.bilibili.com/video/BV1F34y147V7) or try [live demo](https://github.com/calcit-lang/explain-ternary-tree).

![ternary-tree illustrated](assets/ternary-tree-demo.jpeg)

### Usage

![crate](https://img.shields.io/crates/v/im_ternary_tree?style=flat-square)

Docs https://docs.rs/im_ternary_tree/ .

```rust
use im_ternary_tree::TernaryTreeList;

println!("{}", TernaryTreeList::<usize>::from(&[]));

// assoc
let origin5 = [1, 2, 3, 4, 5];
let data5 = TernaryTreeList::from(&origin5);
let updated = data5.assoc(3, 10);

println!("{}", data5.format_inline());
println!("{}", updated.format_inline());

assert_eq!(updated.unsafe_get(3), 10);
```

### Optimizations

方案设计的中文介绍 https://www.bilibili.com/video/BV1z44y1a7a6/

This library has special optimizations on `push_right` and `pop_left` with tricks from [finger-tree](https://en.wikipedia.org/wiki/Finger_tree).

And as its size grows, it always operates on a shallow branch at right end, wasting fewer nodes for indexing new elements, a random demo looks like:

![ternary-tree illustrated](assets/partial.png)

Also the left branches are kept shallow on purpose so it can be cheaper in `pop_left`. This trick is inspired by finger-tree.

### Performance

Benchmarks comparing `TernaryTreeList` with `std::vec::Vec` and `std::collections::VecDeque` show a clear performance profile. As an immutable data structure, `TernaryTreeList` has some overhead compared to its mutable counterparts, but offers significant advantages in specific scenarios.

- **`push_right` / `drop_right` (Appending/Popping from the tail):**
  - `Vec` and `VecDeque` are significantly faster, as they are mutable and optimized for O(1) amortized operations at the tail.
  - `TernaryTreeList` is slower due to the nature of immutable structures, which require creating new tree nodes.

- **`push_left` / `drop_left` (Prepending/Popping from the head):**
  - `TernaryTreeList` is **dramatically faster than `Vec`**. `Vec::insert(0, ...)` is an O(n) operation, while `TernaryTreeList`'s finger-tree-inspired optimizations make head operations much more efficient.
  - `VecDeque` is still the fastest, as it is a mutable data structure specifically designed for O(1) head and tail operations.

**Conclusion:**

- **Use `TernaryTreeList` when:**
  - You need an **immutable** (persistent) list.
  - You require **efficient push and pop operations on both ends** of the list.
- **Use `Vec` or `VecDeque` when:**
  - Mutability is acceptable.
  - You need the absolute best performance for purely mutable operations.

### Known Issues

- lack of optimizations on `pop_right`.
- elements in the middle may be deeply nested, resulting in slow performance.

### License

MIT
