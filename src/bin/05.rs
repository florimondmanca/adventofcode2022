use itertools::Itertools;

// Supply Stacks
// https://adventofcode.com/2022/day/5

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part1, input);
    advent_of_code::solve!(2, part2, input);
}

fn part1(input: &str) -> Option<String> {
    Some(solve(input, apply_crate_mover_9000))
}

fn part2(input: &str) -> Option<String> {
    Some(solve(input, apply_crate_mover_9001))
}

type Stacks = Vec<Vec<char>>;

struct Move {
    quantity: u32,
    source: usize,
    dest: usize,
}

fn solve(content: &str, apply_move: fn(Move, &mut Stacks)) -> String {
    let (drawing, moves) = content.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(drawing);

    moves
        .lines()
        .map(parse_move)
        .for_each(|mv| apply_move(mv, &mut stacks));

    return get_top_crates(stacks);
}

fn parse_stacks(drawing: &str) -> Stacks {
    let reversed_drawing = drawing
        .lines()
        .rev()
        .map(|line| format!("{line}\n"))
        .collect::<String>();

    let (heading, content) = reversed_drawing.split_once("\n").unwrap();

    let num_stacks = heading
        .split(" ")
        .filter(|c| !c.is_empty())
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut stacks: Stacks = Vec::from_iter((0..num_stacks).map(|_| Vec::new()));

    content.lines().for_each(|line| {
        line.chars()
            .chunks(4)
            .into_iter()
            .map(|chunk| chunk.collect::<String>().replace(" ", ""))
            .enumerate()
            .filter(|(_, chunk)| !chunk.is_empty())
            .for_each(|(idx, chunk)| {
                let crate_id = chunk.chars().nth(1).unwrap();
                stacks[idx].push(crate_id);
            });
    });

    return stacks;
}

fn parse_move(line: &str) -> Move {
    let mut it = line.split(" ").into_iter();

    assert!(it.next() == Some("move"));
    let quantity = it.next().unwrap().parse::<u32>().unwrap();
    assert!(it.next() == Some("from"));
    let source = it.next().unwrap().parse::<usize>().unwrap();
    assert!(it.next() == Some("to"));
    let dest = it.next().unwrap().parse::<usize>().unwrap();

    return Move {
        quantity,
        source: source - 1,
        dest: dest - 1,
    };
}

fn apply_crate_mover_9000(mv: Move, stacks: &mut Stacks) {
    for _ in 0..mv.quantity {
        let item = stacks[mv.source].pop().unwrap();
        stacks[mv.dest].push(item);
    }
}

fn apply_crate_mover_9001(mv: Move, stacks: &mut Stacks) {
    let mut items = Vec::new();

    for _ in 0..mv.quantity {
        let item = stacks[mv.source].pop().unwrap();
        items.push(item);
    }

    for _ in 0..mv.quantity {
        let item = items.pop().unwrap();
        stacks[mv.dest].push(item);
    }
}

fn get_top_crates(stacks: Stacks) -> String {
    return stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();
}

#[test]
fn test_part1() {
    let input = &advent_of_code::read_file("inputs", 5);
    assert_eq!(part1(input), Some("SPFMVDTZT".to_string()));
}

#[test]
fn test_part2() {
    let input = &advent_of_code::read_file("inputs", 5);
    assert_eq!(part2(input), Some("ZFSJBPRFP".to_string()));
}
