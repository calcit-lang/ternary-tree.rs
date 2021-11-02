use criterion::{black_box, criterion_group, criterion_main, Criterion};

use im_ternary_tree::TernaryTreeList;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("creating list", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..1000 {
        data = data.push(idx)
      }
    })
  });

  c.bench_function("inserting in middle", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..1000 {
        let pos = idx / 2;
        data = data.insert(pos, idx, false)
      }
    })
  });

  c.bench_function("inserting in middle without balancing", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..1000 {
        let pos = idx / 2;
        data = data.insert(pos, idx, true)
      }
    })
  });

  c.bench_function("rest", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..1000 {
      let pos = idx / 2;
      data = data.insert(pos, idx, false)
    }

    b.iter(move || {
      let mut d = data.to_owned();

      while !d.is_empty() {
        d = d.slice(1, d.len())
      }
    })
  });

  c.bench_function("slice", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..1000 {
      let pos = idx / 2;
      data = data.insert(pos, idx, false)
    }

    b.iter(move || {
      let d = data.to_owned();

      for _ in 0..1000 {
        d.slice(300, 600);
      }
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
