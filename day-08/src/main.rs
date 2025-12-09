use day_08::*;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let input: &str = include_str!("../inputs/input1.txt");

    let solution = part1::process(input, 1000)?;
    println!("Part 1: {}", solution);

    let solution = part2::process(input)?;
    println!("Part 2: {}", solution);

    Ok(())
}
