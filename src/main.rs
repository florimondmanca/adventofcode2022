use std::error::Error;

mod day1;
mod day2;

fn title(s: &str) {
    println!("\n{}", s);
}

fn main() -> Result<(), Box<dyn Error>> {
    title("Day 1: Calories");
    day1::run()?;

    title("Day 2: Rock Paper Scissors");
    day2::run()?;

    Ok(())
}
