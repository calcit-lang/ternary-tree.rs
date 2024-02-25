use criterion::{criterion_group, criterion_main, Criterion};

use im_ternary_tree::TernaryTreeList;

const ITER_SIZE: usize = 10000;

fn criterion_benchmark(c: &mut Criterion) {
  let mut data = TernaryTreeList::Empty;

  for idx in 0..ITER_SIZE {
    data = data.push(idx)
  }

  c.bench_function("iter", |b| {
    let mut cc = 0;

    b.iter(|| {
      for item in &data {
        cc += item;
      }
    })
  });

  c.bench_function("traverse", |b| {
    let mut cc = 0;

    b.iter(|| {
      data.traverse(&mut |item| {
        cc += item;
      });
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
