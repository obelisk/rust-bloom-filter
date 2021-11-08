use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bloomfilter::Bloom;
use rand::prelude::*;

fn check_and_set(values: &[u64]) {
    let mut bloom: Bloom<u64> = Bloom::new_for_fp_rate(100000, 0.001);
    for value in values {
        bloom.check_and_set(value);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // Generate 10,000 values so we don't include this time in our benchmark
    let mut values = [0u64; 10000];
    for i in 0..10000 {
        values[i] = rng.gen_range(0..u64::MAX);
    }

    c.bench_function("check and set 10000 values", |b| b.iter(|| check_and_set(black_box(&values))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);