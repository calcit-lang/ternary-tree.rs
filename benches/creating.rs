use criterion::{criterion_group, criterion_main, Criterion};

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

  c.bench_function("push_right list", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..1000 {
        data = data.push_right(idx)
      }
    })
  });

  c.bench_function("inserting in middle", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..1000 {
        let pos = idx / 2;
        data = data.insert(pos, idx, false).unwrap()
      }
    })
  });

  c.bench_function("inserting in middle without balancing", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..1000 {
        let pos = idx / 2;
        data = data.insert(pos, idx, true).unwrap()
      }
    })
  });

  c.bench_function("rest", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..1000 {
      let pos = idx / 2;
      data = data.insert(pos, idx, false).unwrap()
    }

    b.iter(move || {
      let mut d = data.to_owned();

      while !d.is_empty() {
        d = d.slice(1, d.len()).unwrap()
      }
    })
  });

  c.bench_function("slice", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..1000 {
      let pos = idx / 2;
      data = data.insert(pos, idx, false).unwrap()
    }

    b.iter(move || {
      let d = data.to_owned();

      for _ in 0..1000 {
        d.slice(300, 600).unwrap();
      }
    })
  });

  c.bench_function("index", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..1000 {
      data = data.push(idx)
    }

    b.iter(|| {
      for idx in 0..1000 {
        let _ = data[idx];
      }
    })
  });

  c.bench_function("get", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..1000 {
      data = data.push(idx)
    }

    b.iter(|| {
      for idx in 0..1000 {
        let _ = data.get(idx);
      }
    })
  });

  c.bench_function("loop_get", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..1000 {
      data = data.push(idx)
    }

    b.iter(|| {
      for idx in 0..1000 {
        let _ = data.loop_get(idx);
      }
    })
  });

  c.bench_function("ref_get", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..1000 {
      data = data.push(idx)
    }

    b.iter(|| {
      for idx in 0..1000 {
        let _ = data.ref_get(idx).to_owned();
      }
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
