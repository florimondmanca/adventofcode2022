mod day1;
mod day2;
mod day3;

fn title(s: &str) {
    println!("\n{}", s);
}

fn main() {
    title("Day 1: Calories");
    day1::run();

    title("Day 2: Rock Paper Scissors");
    day2::run();

    title("Day 3: Rucksack Reorganization");
    day3::run();
}
