use criterion::{Criterion, criterion_group, criterion_main};

use im_ternary_tree::TernaryTreeList;

const ITER_SIZE: usize = 10000;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("index", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push(idx)
    }

    b.iter(|| {
      for idx in 0..ITER_SIZE {
        let _ = data[idx];
      }
    })
  });

  c.bench_function("get", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push(idx)
    }

    b.iter(|| {
      for idx in 0..ITER_SIZE {
        let _ = data.get(idx);
      }
    })
  });

  c.bench_function("loop_get", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push(idx)
    }

    b.iter(|| {
      for idx in 0..ITER_SIZE {
        let _ = data.loop_get(idx);
      }
    })
  });

  c.bench_function("ref_get", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push(idx)
    }

    b.iter(|| {
      for idx in 0..ITER_SIZE {
        let _ = data.ref_get(idx).to_owned();
      }
    })
  });

  c.bench_function("first", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push(idx)
    }

    b.iter(|| {
      for _ in 0..ITER_SIZE {
        let _ = data.first();
      }
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
