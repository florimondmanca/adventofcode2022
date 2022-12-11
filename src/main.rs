mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn title(s: &str) {
    println!("\n{}", s);
}

fn main() {
    title("Day 1: Calories");
    day01::run();

    title("Day 2: Rock Paper Scissors");
    day02::run();

    title("Day 3: Rucksack Reorganization");
    day03::run();

    title("Day 4: Camp Cleanup");
    day04::run();

    title("Day 5: Supply Stacks");
    day05::run();

    title("Day 6: Tuning Trouble");
    day06::run();

    title("Day 7: No Space Left On Device");
    day07::run();

    title("Day 8: Treetop Tree House");
    day08::run();

    title("Day 9: Rope Bridge");
    day09::run();

    title("Day 10: Cathode-Ray Tube");
    day10::run();

    title("Day 11: Monkey In The Middle");
    day11::run();
}
