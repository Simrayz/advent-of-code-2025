use day_12::*;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let input: &str = include_str!("../inputs/input1.txt");

    let solution = part1::process(input)?;
    println!("Part 1: {}", solution);

    println!("Part 2: Merry Christmas!");

    Ok(())
}
