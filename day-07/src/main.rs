use day_07::*;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let input: &str = include_str!("../inputs/input1.txt");

    let solution = part1::process_fast(input)?;
    println!("Part 1: {}", solution);

    let solution = part2::process_fast(input)?;
    println!("Part 2: {}", solution);

    Ok(())
}
