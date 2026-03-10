use criterion::{Criterion, criterion_group, criterion_main};
use im_ternary_tree::TernaryTreeList;
use std::collections::VecDeque;
use std::hint::black_box;

const ITER_SIZE: usize = 10000;

fn push_right_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("push_right");

  group.bench_function("TernaryTreeList", |b| {
    b.iter(|| {
      let mut list = TernaryTreeList::Empty;
      for i in 0..ITER_SIZE {
        list = list.push_right(black_box(i));
      }
      black_box(list.len());
    })
  });

  group.bench_function("Vec", |b| {
    b.iter(|| {
      let mut list = Vec::new();
      for i in 0..ITER_SIZE {
        list.push(black_box(i));
      }
      black_box(list.len());
    })
  });

  group.bench_function("VecDeque", |b| {
    b.iter(|| {
      let mut list = VecDeque::new();
      for i in 0..ITER_SIZE {
        list.push_back(black_box(i));
      }
      black_box(list.len());
    })
  });

  group.finish();
}

fn push_left_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("push_left");

  group.bench_function("TernaryTreeList", |b| {
    b.iter(|| {
      let mut list = TernaryTreeList::Empty;
      for i in 0..ITER_SIZE {
        list = list.push_left(black_box(i));
      }
      black_box(list.len());
    })
  });

  group.bench_function("Vec", |b| {
    b.iter(|| {
      let mut list = Vec::new();
      for i in 0..ITER_SIZE {
        list.insert(0, black_box(i));
      }
      black_box(list.len());
    })
  });

  group.bench_function("VecDeque", |b| {
    b.iter(|| {
      let mut list = VecDeque::new();
      for i in 0..ITER_SIZE {
        list.push_front(black_box(i));
      }
      black_box(list.len());
    })
  });

  group.finish();
}

fn drop_left_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("drop_left");

  let tree = (0..ITER_SIZE).fold(TernaryTreeList::Empty, |acc, i| acc.push_right(i));
  group.bench_function("TernaryTreeList", |b| {
    b.iter(|| {
      let mut list = tree.clone();
      for _ in 0..ITER_SIZE {
        list = list.drop_left();
      }
      black_box(list.len());
    })
  });

  let vec = (0..ITER_SIZE).collect::<Vec<_>>();
  group.bench_function("Vec", |b| {
    b.iter(|| {
      let mut list = vec.clone();
      for _ in 0..ITER_SIZE {
        list.remove(0);
      }
      black_box(list.len());
    })
  });

  let deque = (0..ITER_SIZE).collect::<VecDeque<_>>();
  group.bench_function("VecDeque", |b| {
    b.iter(|| {
      let mut list = deque.clone();
      for _ in 0..ITER_SIZE {
        list.pop_front();
      }
      black_box(list.len());
    })
  });

  group.finish();
}

fn drop_right_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("drop_right");

  let tree = (0..ITER_SIZE).fold(TernaryTreeList::Empty, |acc, i| acc.push_right(i));
  group.bench_function("TernaryTreeList", |b| {
    b.iter(|| {
      let mut list = tree.clone();
      for _ in 0..ITER_SIZE {
        list = list.drop_right();
      }
      black_box(list.len());
    })
  });

  let vec = (0..ITER_SIZE).collect::<Vec<_>>();
  group.bench_function("Vec", |b| {
    b.iter(|| {
      let mut list = vec.clone();
      for _ in 0..ITER_SIZE {
        list.pop();
      }
      black_box(list.len());
    })
  });

  let deque = (0..ITER_SIZE).collect::<VecDeque<_>>();
  group.bench_function("VecDeque", |b| {
    b.iter(|| {
      let mut list = deque.clone();
      for _ in 0..ITER_SIZE {
        list.pop_back();
      }
      black_box(list.len());
    })
  });

  group.finish();
}

criterion_group!(
  benches,
  push_right_benchmark,
  push_left_benchmark,
  drop_left_benchmark,
  drop_right_benchmark
);
criterion_main!(benches);
