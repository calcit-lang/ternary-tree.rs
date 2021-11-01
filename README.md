## Structrual sharing Ternary Tree in Rust

> a bit like 2-3 finger three, but not very like... WIP

### Usage

_TODO_

```rust
use im_ternary_tree::TernaryTreeList;

println!("{}", TernaryTreeList::<usize>::init_from(&[]));

// assoc
let origin5 = vec![1, 2, 3, 4, 5];
let data5 = TernaryTreeList::init_from(&origin5);
let updated = data5.assoc(3, 10);

println!("{}", data5.format_inline());
println!("{}", updated.format_inline());

assert_eq!(updated.unsafe_get(3), 10);
```

### Known Issues

- Balancing is not ensured, thus unstable performance

### License

MIT
