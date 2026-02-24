use broken_app::{algo, sum_even};
use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};
use std::time::Duration;

fn bench_sum_even(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_even");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(30);

    for size in [50_000_usize, 1_000_000_usize] {
        let data: Vec<i64> = (0..size as i64).collect();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, input| {
            b.iter(|| black_box(sum_even(black_box(input))))
        });
    }

    group.finish();
}

fn bench_fib(c: &mut Criterion) {
    let mut group = c.benchmark_group("slow_fib");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(20);

    for n in [28_u64, 32_u64] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, input| {
            b.iter(|| black_box(algo::slow_fib(black_box(*input))))
        });
    }

    group.finish();
}

fn bench_dedup(c: &mut Criterion) {
    let mut group = c.benchmark_group("slow_dedup");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(20);

    for unique in [5_000_u64, 20_000_u64] {
        let data: Vec<u64> = (0..unique).flat_map(|n| [n, n]).collect();
        group.throughput(Throughput::Elements(data.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(unique), &data, |b, input| {
            b.iter_batched(
                || input.clone(),
                |v| black_box(algo::slow_dedup(black_box(&v))),
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sum_even, bench_fib, bench_dedup);
criterion_main!(benches);
