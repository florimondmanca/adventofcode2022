mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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

    title("Day 6: Tuning Trouble");
    day6::run();

    title("Day 7: No Space Left On Device");
    day7::run();

    title("Day 8: Treetop Tree House");
    day8::run();

    title("Day 9: Rope Bridge");
    day9::run();

    title("Day 10: Cathode-Ray Tube");
    day10::run();
}
