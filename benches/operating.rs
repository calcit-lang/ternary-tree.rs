use criterion::{Criterion, criterion_group, criterion_main};

use im_ternary_tree::TernaryTreeList;

const ITER_SIZE: usize = 10000;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("append to list", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..ITER_SIZE {
        data = data.append(idx)
      }
    })
  });

  c.bench_function("push_right list", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..ITER_SIZE {
        data = data.push_right(idx)
      }
    })
  });

  c.bench_function("unshift to list", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..ITER_SIZE {
        data = data.unshift(idx)
      }
    })
  });

  c.bench_function("push_left list", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      for idx in 0..ITER_SIZE {
        data = data.push_left(idx)
      }
    })
  });

  c.bench_function("inserting in middle", |b| {
    b.iter(|| {
      let mut data = TernaryTreeList::Empty;

      // TODO overflowed
      for idx in 0..ITER_SIZE {
        let pos = idx / 2;
        data = data.insert(pos, idx, false).unwrap()
      }
    })
  });

  c.bench_function("rest", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push(idx);
    }

    b.iter(move || {
      let mut d = data.to_owned();

      while !d.is_empty() {
        d = d.slice(1, d.len()).unwrap()
      }
    })
  });

  c.bench_function("rest from push_right", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push_right(idx);
    }

    b.iter(move || {
      let mut d = data.to_owned();

      while !d.is_empty() {
        d = d.slice(1, d.len()).unwrap()
      }
    })
  });

  c.bench_function("drop-left", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push(idx);
    }

    b.iter(move || {
      let mut d = data.to_owned();

      while d.len() > 1 {
        d = d.drop_left()
      }
    })
  });

  c.bench_function("drop-right", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push(idx);
    }

    b.iter(move || {
      let mut d = data.to_owned();

      while d.len() > 1 {
        d = d.drop_right()
      }
    })
  });

  c.bench_function("drop-left-shallow", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push(idx);
    }

    b.iter(move || {
      let mut d = data.to_owned();

      while d.len() > 1 {
        d = d.drop_left_shallow()
      }
    })
  });

  c.bench_function("drop-right-shallow", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push(idx);
    }

    b.iter(move || {
      let mut d = data.to_owned();

      while d.len() > 1 {
        d = d.drop_right_shallow()
      }
    })
  });

  c.bench_function("drop_left from push_right", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push_right(idx);
    }

    b.iter(move || {
      let mut d = data.to_owned();

      while d.len() > 1 {
        d = d.drop_left()
      }
    })
  });

  c.bench_function("slice 300-600", |b| {
    let mut data = TernaryTreeList::Empty;

    for idx in 0..ITER_SIZE {
      data = data.push(idx)
    }

    b.iter(move || {
      let d = data.to_owned();

      for _ in 0..ITER_SIZE {
        d.slice(300, 600).unwrap();
      }
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
