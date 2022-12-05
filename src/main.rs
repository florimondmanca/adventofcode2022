mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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

    title("Day 4: Camp Cleanup");
    day4::run();

    title("Day 5: Supply Stacks");
    day5::run();
}
