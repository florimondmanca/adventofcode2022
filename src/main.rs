use std::error::Error;

mod day1;
mod day2;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 1: Calories -- https://adventofcode.com/2022/day/1");
    day1::run()?;

    println!("Day 2: Rock Paper Scissors -- https://adventofcode.com/2022/day/2");
    day2::run()?;

    Ok(())
}
