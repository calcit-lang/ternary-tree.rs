use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use im_ternary_tree::TernaryTreeList;
use std::hint::black_box;

fn concat_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("concat");

    // Case 1: Balanced - 100 lists of 100 elements
    let list_count_balanced = 100;
    let list_size_balanced = 100;
    let balanced_lists = (0..list_count_balanced)
        .map(|_| TernaryTreeList::from((0..list_size_balanced).collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    group.bench_with_input(
        BenchmarkId::new("Balanced (New)", format!("{}x{}", list_count_balanced, list_size_balanced)),
        &balanced_lists,
        |b, lists| {
            b.iter(|| TernaryTreeList::concat(black_box(lists)));
        },
    );
    group.bench_with_input(
        BenchmarkId::new("Balanced (Dumb)", format!("{}x{}", list_count_balanced, list_size_balanced)),
        &balanced_lists,
        |b, lists| {
            b.iter(|| TernaryTreeList::concat_dumb(black_box(lists)));
        },
    );

    // Case 2: Many Shallow - 1000 lists of 10 elements
    let list_count_shallow = 1000;
    let list_size_shallow = 10;
    let shallow_lists = (0..list_count_shallow)
        .map(|_| TernaryTreeList::from((0..list_size_shallow).collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    group.bench_with_input(
        BenchmarkId::new("ManyShallow (New)", format!("{}x{}", list_count_shallow, list_size_shallow)),
        &shallow_lists,
        |b, lists| {
            b.iter(|| TernaryTreeList::concat(black_box(lists)));
        },
    );
    group.bench_with_input(
        BenchmarkId::new("ManyShallow (Dumb)", format!("{}x{}", list_count_shallow, list_size_shallow)),
        &shallow_lists,
        |b, lists| {
            b.iter(|| TernaryTreeList::concat_dumb(black_box(lists)));
        },
    );

    // Case 3: Few Deep - 10 lists of 1000 elements
    let list_count_deep = 10;
    let list_size_deep = 1000;
    let deep_lists = (0..list_count_deep)
        .map(|_| TernaryTreeList::from((0..list_size_deep).collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    group.bench_with_input(
        BenchmarkId::new("FewDeep (New)", format!("{}x{}", list_count_deep, list_size_deep)),
        &deep_lists,
        |b, lists| {
            b.iter(|| TernaryTreeList::concat(black_box(lists)));
        },
    );
    group.bench_with_input(
        BenchmarkId::new("FewDeep (Dumb)", format!("{}x{}", list_count_deep, list_size_deep)),
        &deep_lists,
        |b, lists| {
            b.iter(|| TernaryTreeList::concat_dumb(black_box(lists)));
        },
    );

    group.finish();
}

criterion_group!(benches, concat_benchmark);
criterion_main!(benches);