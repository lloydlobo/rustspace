use criterion::*;
use std::thread::{self, sleep};
use std::time::{self, Duration};

pub(crate) use lib_time::*;

fn simple_characters_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("\"*group/\"");
    group.bench_function("\"*benchmark/\" '", |b| b.iter(|| 1 + 1));
    group.finish();
}

fn time_delay_test(c: &mut Criterion) {
    let time_ms = 10;
    let ten_millis = time::Duration::from_millis(time_ms);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
    println!("now({}): {:?}", time_ms, now);
    let now = time::Instant::now();
    sleep(Duration::from_millis(time_ms));
    println!("now({}): {:?}", time_ms, now);
    c.bench_function("forced_zero_time_test_0", |b| {
        b.iter(|| Duration::new(0, 0))
    });
}

fn lib_sqrt_tests(c: &mut Criterion) {
    let mut group = c.benchmark_group("SystemTime");
    group.bench_function("Bench 1: SystemTime Mod", |b| {
        b.iter(bench_lib_now);
    });
    group.bench_function("Bench 2: SystemTime Inline", |b| {
        b.iter(bench_lib_now_inline);
    });

    group.finish();
}

fn bench_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("My Group");
    // Now we can perform benchmarks with this group
    group.bench_function("Bench 1", |b| b.iter(|| 1));
    group.bench_function("Bench 2", |b| b.iter(|| 2));

    group.finish();
}

fn zero_test(c: &mut Criterion) {
    c.bench_function("forced_zero_time_test", |b| {
        b.iter_custom(|_iters| Duration::new(0, 0))
    });
}

criterion_group!(
    benches,
    simple_characters_test,
    time_delay_test,
    lib_sqrt_tests,
    bench_simple,
    zero_test
);
criterion_main!(benches);

// fn bench_simple(c: &mut Criterion) {
//     let mut group = c.benchmark_group("My Group");
//     // Now we can perform benchmarks with this group
//     group.bench_function("Bench 1", |b| b.iter(|| 1));
//     group.bench_function("Bench 2", |b| b.iter(|| 2));
//     group.finish();
// }

// criterion_group!(benches, sort_arr_benchmark, fibonacci_sequence_benchmark);
// criterion_group! {
//     name = benches;
//     config = Criterion::default();
//     targets = sort_arr_benchmark, fibonacci_sequence_benchmark
// }
