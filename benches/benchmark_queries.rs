use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tree_sitter::Query;

fn benchmark(criterion: &mut Criterion) {
    let query = include_str!("../queries/highlights.scm");
    let language = experimental_tree_sitter_swift::language();

    let mut group = criterion.benchmark_group("highlights");
    group.sample_size(20);

    group.bench_function("ts_query_new", |b| b.iter(|| {
        let _ = Query::new(
            black_box(language),
            black_box(query)
        );
    }));

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
