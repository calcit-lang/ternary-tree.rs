## Structrual sharing tree in Rust

https://img.shields.io/crates/v/im_ternary_tree?style=flat-square

> Actually an unbalanced 2-3 tree with tricks like finger-tree.

### Usage

![](https://img.shields.io/crates/v/im_ternary_tree?style=flat-square)

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

Videos:

- tree layout from 1 to 59 https://www.bilibili.com/video/BV1or4y1U7u2/
- ideas explained in Chinese https://www.bilibili.com/video/BV1z44y1a7a6/

This library has special optimizations on `push_right` and `pop_left` with tricks from [finger-tree](https://en.wikipedia.org/wiki/Finger_tree).

For a vector of `([] 0 1 2 3 4 5 6 7)`, its internal structure is like:

```cirru
((0 1 2) (3 4 5) (6 7))
```

each pair of `(` and `)` represents a branch, notice that each group is size 1, 2 or 3.

And as its size grows, it's always operating on a shallow branch at right end, wasting fewer nodes for indexing new elements:

```cirru
  0
 (0 1)
 (0 1 2)
((0 1 2) 3)
((0 1 2) (3 4))
((0 1 2) (3 4 5))
((0 1 2) (3 4 5) 6)
((0 1 2) (3 4 5) (6 7))
((0 1 2) (3 4 5) (6 7 8))
((0 1 2) ((3 4 5) (6 7 8)) 9)
((0 1 2) ((3 4 5) (6 7 8)) (9 10))
((0 1 2) ((3 4 5) (6 7 8)) (9 10 11))
((0 1 2) ((3 4 5) (6 7 8) (9 10 11)) 12)
((0 1 2) ((3 4 5) (6 7 8) (9 10 11)) (12 13))
((0 1 2) ((3 4 5) (6 7 8) (9 10 11)) (12 13 14))
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) (12 13 14)) 15)
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) (12 13 14)) (15 16))
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) (12 13 14)) (15 16 17))
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) ((12 13 14) (15 16 17))) 18)
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) ((12 13 14) (15 16 17))) (18 19))
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) ((12 13 14) (15 16 17))) (18 19 20))
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) ((12 13 14) (15 16 17) (18 19 20))) 21)
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) ((12 13 14) (15 16 17) (18 19 20))) (21 22))
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) ((12 13 14) (15 16 17) (18 19 20))) (21 22 23))
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) ((12 13 14) (15 16 17) (18 19 20)) (21 22 23)) 24)
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) ((12 13 14) (15 16 17) (18 19 20)) (21 22 23)) (24 25))
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) ((12 13 14) (15 16 17) (18 19 20)) (21 22 23)) (24 25 26))
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) ((12 13 14) (15 16 17) (18 19 20)) ((21 22 23) (24 25 26))) 27)
((0 1 2) (((3 4 5) (6 7 8) (9 10 11)) ((12 13 14) (15 16 17) (18 19 20)) ((21 22 23) (24 25 26))) (27 28))
```

Also the left branches are kept shallow on purpose so it can be cheaper in `pop_left`. Totally inspired by finger-tree.

### Known Issues

- no optimizations on `pop_right` and `push_left`.
- elements in the middle could be inside deep branches, leading to slow performance.

### License

MIT
