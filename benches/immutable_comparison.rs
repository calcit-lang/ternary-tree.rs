use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use im_ternary_tree::TernaryTreeList;
use rpds::{ListSync, VectorSync};
use std::hint::black_box;

const SIZE: usize = 10_000;
const UPDATE_COUNT: usize = 1_000;

fn build_ternary() -> TernaryTreeList<usize> {
  TernaryTreeList::from((0..SIZE).collect::<Vec<_>>())
}

fn build_rpds_vector() -> VectorSync<usize> {
  let mut vector = VectorSync::new_sync();
  for value in 0..SIZE {
    vector.push_back_mut(value);
  }
  vector
}

fn build_rpds_list() -> ListSync<usize> {
  let mut list = ListSync::new_sync();
  for value in 0..SIZE {
    list.push_front_mut(value);
  }
  list
}

fn bulk_build(c: &mut Criterion) {
  let mut group = c.benchmark_group("immutable/bulk_build");
  group.throughput(Throughput::Elements(SIZE as u64));
  let values = (0..SIZE).collect::<Vec<_>>();

  group.bench_function("ternary_from_vec", |b| {
    b.iter_batched(
      || values.clone(),
      |input| black_box(TernaryTreeList::from(black_box(input))),
      BatchSize::SmallInput,
    )
  });
  group.bench_function("rpds_vector_from_iter", |b| {
    b.iter_batched(
      || values.clone(),
      |input| black_box(input.into_iter().collect::<VectorSync<_>>()),
      BatchSize::SmallInput,
    )
  });
  group.finish();
}

fn persistent_push(c: &mut Criterion) {
  let mut group = c.benchmark_group("immutable/persistent_push");
  group.throughput(Throughput::Elements(SIZE as u64));

  group.bench_function("ternary_push_right", |b| {
    b.iter(|| {
      let mut list = TernaryTreeList::Empty;
      for value in 0..SIZE {
        list = list.push_right(black_box(value));
      }
      black_box(list)
    })
  });
  group.bench_function("rpds_vector_push_back", |b| {
    b.iter(|| {
      let mut vector = VectorSync::new_sync();
      for value in 0..SIZE {
        vector = vector.push_back(black_box(value));
      }
      black_box(vector)
    })
  });
  group.bench_function("ternary_push_left", |b| {
    b.iter(|| {
      let mut list = TernaryTreeList::Empty;
      for value in 0..SIZE {
        list = list.push_left(black_box(value));
      }
      black_box(list)
    })
  });
  group.bench_function("rpds_list_push_front", |b| {
    b.iter(|| {
      let mut list = ListSync::new_sync();
      for value in 0..SIZE {
        list = list.push_front(black_box(value));
      }
      black_box(list)
    })
  });
  group.finish();
}

fn persistent_update(c: &mut Criterion) {
  let mut group = c.benchmark_group("immutable/persistent_update");
  group.throughput(Throughput::Elements(UPDATE_COUNT as u64));
  let ternary = build_ternary();
  let vector = build_rpds_vector();
  let indices = (0..UPDATE_COUNT).map(|i| (i.wrapping_mul(7919)) % SIZE).collect::<Vec<_>>();

  group.bench_function("ternary_assoc", |b| {
    b.iter(|| {
      for &idx in &indices {
        black_box(ternary.assoc(idx, black_box(idx + 1)).unwrap());
      }
    })
  });
  group.bench_function("rpds_vector_set", |b| {
    b.iter(|| {
      for &idx in &indices {
        black_box(vector.set(idx, black_box(idx + 1)).unwrap());
      }
    })
  });
  group.finish();
}

fn random_access(c: &mut Criterion) {
  let mut group = c.benchmark_group("immutable/random_access");
  group.throughput(Throughput::Elements(SIZE as u64));
  let ternary = build_ternary();
  let vector = build_rpds_vector();
  let indices = (0..SIZE).map(|i| (i.wrapping_mul(7919)) % SIZE).collect::<Vec<_>>();

  group.bench_function("ternary_get", |b| {
    b.iter(|| {
      for &idx in &indices {
        black_box(ternary.get(idx));
      }
    })
  });
  group.bench_function("rpds_vector_get", |b| {
    b.iter(|| {
      for &idx in &indices {
        black_box(vector.get(idx));
      }
    })
  });
  group.finish();
}

fn iteration(c: &mut Criterion) {
  let mut group = c.benchmark_group("immutable/iteration");
  group.throughput(Throughput::Elements(SIZE as u64));
  let ternary = build_ternary();
  let vector = build_rpds_vector();
  let list = build_rpds_list();

  group.bench_function("ternary", |b| b.iter(|| black_box(ternary.iter().copied().sum::<usize>())));
  group.bench_function("rpds_vector", |b| b.iter(|| black_box(vector.iter().copied().sum::<usize>())));
  group.bench_function("rpds_list", |b| b.iter(|| black_box(list.iter().copied().sum::<usize>())));
  group.finish();
}

fn persistent_drop(c: &mut Criterion) {
  let mut group = c.benchmark_group("immutable/persistent_drop");
  group.throughput(Throughput::Elements(SIZE as u64));
  let ternary = build_ternary();
  let vector = build_rpds_vector();
  let list = build_rpds_list();

  group.bench_function("ternary_drop_right", |b| {
    b.iter(|| {
      let mut current = ternary.clone();
      while !current.is_empty() {
        current = current.drop_right();
      }
      black_box(current)
    })
  });
  group.bench_function("rpds_vector_drop_last", |b| {
    b.iter(|| {
      let mut current = vector.clone();
      while let Some(next) = current.drop_last() {
        current = next;
      }
      black_box(current)
    })
  });
  group.bench_function("ternary_drop_left", |b| {
    b.iter(|| {
      let mut current = ternary.clone();
      while !current.is_empty() {
        current = current.drop_left();
      }
      black_box(current)
    })
  });
  group.bench_function("rpds_list_drop_first", |b| {
    b.iter(|| {
      let mut current = list.clone();
      while let Some(next) = current.drop_first() {
        current = next;
      }
      black_box(current)
    })
  });
  group.finish();
}

criterion_group!(
  benches,
  bulk_build,
  persistent_push,
  persistent_update,
  random_access,
  iteration,
  persistent_drop,
);
criterion_main!(benches);
