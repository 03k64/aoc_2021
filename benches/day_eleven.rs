use aoc_2021::day_eleven::calculate_flashes;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn use_example_input() -> Vec<String> {
    String::from(
        r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#,
    )
    .lines()
    .map(String::from)
    .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("calculate_flashes example_input", |b| {
        b.iter(|| calculate_flashes(black_box(use_example_input()), black_box(100)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
