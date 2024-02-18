use criterion::{criterion_group, criterion_main, Criterion};

use im_ternary_tree::TernaryTreeList;

const ITER_SIZE: usize = 10000;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("creating list", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..ITER_SIZE {
        data = data.push(idx)
      }
    })
  });

  c.bench_function("creating list disable balancing", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..ITER_SIZE {
        data = data.append(idx)
      }
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
