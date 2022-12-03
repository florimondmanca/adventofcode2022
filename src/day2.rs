use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

fn get_shape_score(s: Shape) -> i32 {
    return match s {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };
}

fn get_outcome(opponent: Shape, player: Shape) -> Outcome {
    return match (opponent, player) {
        (Shape::Rock, Shape::Rock) => Outcome::Draw,
        (Shape::Rock, Shape::Paper) => Outcome::Win,
        (Shape::Rock, Shape::Scissors) => Outcome::Lose,
        (Shape::Paper, Shape::Rock) => Outcome::Lose,
        (Shape::Paper, Shape::Paper) => Outcome::Draw,
        (Shape::Paper, Shape::Scissors) => Outcome::Win,
        (Shape::Scissors, Shape::Rock) => Outcome::Win,
        (Shape::Scissors, Shape::Paper) => Outcome::Lose,
        (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
    };
}

fn get_outcome_score(outcome: Outcome) -> i32 {
    return match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    };
}

fn part1(content: String) -> Result<(), Box<dyn Error>> {
    let mut score = 0;

    for line in content.lines() {
        let opponent = match line.chars().nth(0) {
            Some('A') => Shape::Rock,
            Some('B') => Shape::Paper,
            Some('C') => Shape::Scissors,
            _ => panic!("invalid line"),
        };

        let player = match line.chars().nth(2) {
            Some('X') => Shape::Rock,
            Some('Y') => Shape::Paper,
            Some('Z') => Shape::Scissors,
            _ => panic!("invalid line"),
        };

        let outcome = get_outcome(opponent, player.clone());

        score += get_outcome_score(outcome) + get_shape_score(player.clone());
    }

    println!("Answer (part 1): {}", score);

    Ok(())
}

fn choose_move(opponent: Shape, outcome: Outcome) -> Shape {
    return match (opponent, outcome) {
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Rock, Outcome::Lose) => Shape::Scissors,
        (Shape::Rock, Outcome::Draw) => Shape::Rock,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Paper, Outcome::Lose) => Shape::Rock,
        (Shape::Paper, Outcome::Draw) => Shape::Paper,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Scissors, Outcome::Lose) => Shape::Paper,
        (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
    };
}

fn part2(content: String) -> Result<(), Box<dyn Error>> {
    let mut score = 0;

    for line in content.lines() {
        let opponent = match line.chars().nth(0) {
            Some('A') => Shape::Rock,
            Some('B') => Shape::Paper,
            Some('C') => Shape::Scissors,
            _ => panic!("invalid line"),
        };

        let outcome = match line.chars().nth(2) {
            Some('X') => Outcome::Lose,
            Some('Y') => Outcome::Draw,
            Some('Z') => Outcome::Win,
            _ => panic!("invalid line"),
        };

        let player = choose_move(opponent, outcome.clone());

        score += get_outcome_score(outcome.clone()) + get_shape_score(player);
    }

    println!("Answer (part 2): {}", score);

    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(Path::new("inputs/2.txt"))?;
    part1(content.clone())?;
    part2(content.clone())?;
    Ok(())
}
