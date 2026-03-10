use criterion::{Criterion, criterion_group, criterion_main};
use im_ternary_tree::TernaryTreeList;
use std::hint::black_box;
use std::sync::Arc;

const LIST_SIZE: usize = 10000;
const QUERIES: usize = 1000;

fn criterion_benchmark(c: &mut Criterion) {
  let tree = (0..LIST_SIZE).fold(TernaryTreeList::Empty, |acc, i| acc.push_right(i));
  let vec = (0..LIST_SIZE).collect::<Vec<_>>();

  c.bench_function("index_of_hit/tree", |b| {
    b.iter(|| {
      for i in 0..QUERIES {
        let needle = i * (LIST_SIZE / QUERIES);
        black_box(tree.index_of(&needle));
      }
    })
  });

  c.bench_function("index_of_hit/vec", |b| {
    b.iter(|| {
      for i in 0..QUERIES {
        let needle = i * (LIST_SIZE / QUERIES);
        black_box(vec.iter().position(|v| *v == needle));
      }
    })
  });

  c.bench_function("last_index_of_hit/tree", |b| {
    b.iter(|| {
      for i in 0..QUERIES {
        let needle = i * (LIST_SIZE / QUERIES);
        black_box(tree.last_index_of(&needle));
      }
    })
  });

  c.bench_function("last_index_of_hit/vec", |b| {
    b.iter(|| {
      for i in 0..QUERIES {
        let needle = i * (LIST_SIZE / QUERIES);
        black_box(vec.iter().rposition(|v| *v == needle));
      }
    })
  });

  c.bench_function("index_of_miss/tree", |b| {
    b.iter(|| {
      for i in 0..QUERIES {
        black_box(tree.index_of(&(LIST_SIZE + i + 1)));
      }
    })
  });

  c.bench_function("last_index_of_miss/tree", |b| {
    b.iter(|| {
      for i in 0..QUERIES {
        black_box(tree.last_index_of(&(LIST_SIZE + i + 1)));
      }
    })
  });

  c.bench_function("find_index_hit/tree", |b| {
    b.iter(|| {
      for i in 0..QUERIES {
        let needle = i * (LIST_SIZE / QUERIES);
        black_box(tree.find_index(Arc::new(move |v| *v == needle)));
      }
    })
  });

  c.bench_function("find_index_miss/tree", |b| {
    b.iter(|| {
      for i in 0..QUERIES {
        black_box(tree.find_index(Arc::new(move |v| *v == LIST_SIZE + i + 1)));
      }
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
