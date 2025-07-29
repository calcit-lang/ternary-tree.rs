use criterion::{criterion_group, criterion_main, Criterion};
use im_ternary_tree::TernaryTreeList;
use std::collections::VecDeque;
use std::hint::black_box;

const LIST_SIZE: usize = 10000;

fn sequential_access_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_access");

    let tree = (0..LIST_SIZE).fold(TernaryTreeList::Empty, |acc, i| acc.push_right(i));
    group.bench_function("TernaryTreeList", |b| {
        b.iter(|| {
            for i in 0..LIST_SIZE {
                black_box(tree.get(i));
            }
        })
    });

    let vec = (0..LIST_SIZE).collect::<Vec<_>>();
    group.bench_function("Vec", |b| {
        b.iter(|| {
            for i in 0..LIST_SIZE {
                black_box(vec.get(i));
            }
        })
    });

    let deque = (0..LIST_SIZE).collect::<VecDeque<_>>();
    group.bench_function("VecDeque", |b| {
        b.iter(|| {
            for i in 0..LIST_SIZE {
                black_box(deque.get(i));
            }
        })
    });

    group.finish();
}

fn random_access_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_access");

    let mut rand_indices = Vec::with_capacity(LIST_SIZE);
    for _ in 0..LIST_SIZE {
        rand_indices.push(fastrand::usize(0..LIST_SIZE));
    }

    let tree = (0..LIST_SIZE).fold(TernaryTreeList::Empty, |acc, i| acc.push_right(i));
    group.bench_function("TernaryTreeList", |b| {
        b.iter(|| {
            for &i in &rand_indices {
                black_box(tree.get(i));
            }
        })
    });

    let vec = (0..LIST_SIZE).collect::<Vec<_>>();
    group.bench_function("Vec", |b| {
        b.iter(|| {
            for &i in &rand_indices {
                black_box(vec.get(i));
            }
        })
    });

    let deque = (0..LIST_SIZE).collect::<VecDeque<_>>();
    group.bench_function("VecDeque", |b| {
        b.iter(|| {
            for &i in &rand_indices {
                black_box(deque.get(i));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, sequential_access_benchmark, random_access_benchmark);
criterion_main!(benches);
