use itertools::Itertools;

pub fn run() {
    let content = include_str!("inputs/5.txt");

    let (drawing, moves) = content.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(drawing);

    moves
        .lines()
        .map(parse_move)
        .for_each(|mv| apply_move(mv, &mut stacks));

    let message = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("Answer (part 1): {message}");
}

struct Move {
    quantity: u32,
    source: usize,
    dest: usize,
}

fn parse_stacks(drawing: &str) -> Vec<Vec<char>> {
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

    let mut stacks: Vec<Vec<char>> = Vec::from_iter((0..num_stacks).map(|_| Vec::new()));

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
fn apply_move(mv: Move, stacks: &mut Vec<Vec<char>>) {
    for _ in 0..mv.quantity {
        let item = stacks[mv.source].pop().unwrap();
        stacks[mv.dest].push(item);
    }
}
