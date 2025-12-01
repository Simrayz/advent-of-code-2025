use day_01::{part1};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../inputs/example.txt",))).unwrap();
}

