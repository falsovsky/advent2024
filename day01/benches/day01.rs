use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day01::*;
use std::env;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut root = env::current_dir().unwrap();
    root.push("../input/day01.txt");
    c.bench_function("day01 read_input", |b| {
        b.iter(|| read_input(black_box(root.clone())))
    });
    let input = read_input(root);
    c.bench_function("day01 solve_part1", |b| {
        b.iter(|| solve_part1(black_box(&input)))
    });
    c.bench_function("day01 solve_part2", |b| {
        b.iter(|| solve_part2(black_box(&input)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1000);
    targets = criterion_benchmark
}
criterion_main!(benches);
