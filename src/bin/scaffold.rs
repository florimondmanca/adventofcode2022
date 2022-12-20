use std::{env, fs::OpenOptions, io::Write, process};

const TEMPLATE: &str = r###"fn main() {
    println!("Title");
    let input = &advent_of_code::read_file("inputs", $day);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<u32> {
    None
}

fn part2(input: &str) -> Option<u32> {
    None
}
"###;

fn help() -> Option<u32> {
    println!("Usage: cargo scaffold [day:int]");
    None
}

fn parse() -> Option<u32> {
    let args = env::args().collect::<Vec<String>>();

    match args.len() {
        2 => match args[1].parse::<u32>() {
            Ok(day) => Some(day),
            _ => help(),
        },
        _ => help(),
    }
}

fn main() {
    let day = match parse() {
        Some(d) => d,
        None => process::exit(1),
    };

    let bin_path = format!("src/bin/{day:02}.rs");
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(bin_path)
        .expect("Failed to create bin file")
        .write_all(TEMPLATE.replace("$day", &day.to_string()).as_bytes())
        .expect("Failed to write bin file");

    let input_path = format!("src/inputs/{day:02}.txt");
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(input_path)
        .expect("Failed to create input path");
}
