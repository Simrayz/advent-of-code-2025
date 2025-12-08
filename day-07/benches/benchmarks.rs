use day_07::{part1, part2};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../inputs/input1.txt"))).unwrap();
}

#[divan::bench]
fn part1_fast() {
    part1::process_fast(divan::black_box(include_str!("../inputs/input1.txt"))).unwrap();
}


#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../inputs/input1.txt"))).unwrap();
}

#[divan::bench]
fn part2_fast() {
    part2::process_fast(divan::black_box(include_str!("../inputs/input1.txt"))).unwrap();
}
