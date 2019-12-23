use criterion::{black_box, criterion_group, criterion_main, Criterion};
use unquote::tokenize;

fn criterion_benchmark(c: &mut Criterion) {
    let code = include_str!("/usr/local/bin/gitk");
    c.bench_function("gitk", |b| b.iter(|| tokenize(black_box(code))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
