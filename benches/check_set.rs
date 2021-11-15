use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bloomfilter::Bloom;
use rand::{distributions::Alphanumeric, Rng};

use core::hash::Hash;

fn check_and_set<T>(values: &[T]) where T: Hash {
    let mut bloom: Bloom<T> = Bloom::new_for_fp_rate(100000, 0.001);
    for value in values {
        bloom.check_and_set(value);
    }
}

fn bloom_hash<T>(values: &[T]) where T: Hash {
    let mut bloom: Bloom<T> = Bloom::new_for_fp_rate(100000, 0.001);
    let mut hashes = [0u64, 0u64];

    for v in values {
        bloom.bloom_hash(&mut hashes, v, 0);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // Generate 10,000 values so we don't include this time in our benchmark
    let mut u64_values = [0u64; 10000];
    for i in 0..10000 {
        u64_values[i] = rng.gen_range(0..u64::MAX);
    }

    let mut string_values: Vec<String> = vec![String::new(); 10000];
    for i in 0..10000 {
        string_values[i] = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();
    }

    //c.bench_function("check and set 10000 u64 values", |b| b.iter(|| check_and_set(black_box(&u64_values))));
    //c.bench_function("check and set 10000 string values", |b| b.iter(|| check_and_set(black_box(&string_values))));

    c.bench_function("check bloom_hash speed", |b| b.iter(|| bloom_hash(&string_values)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);