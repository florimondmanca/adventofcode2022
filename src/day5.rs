pub fn run() {
    let content = include_str!("inputs/5.txt");

    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.push(vec!['F', 'C', 'J', 'P', 'H', 'T', 'W']);
    stacks.push(vec!['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H']);
    stacks.push(vec!['H', 'P', 'T', 'R']);
    stacks.push(vec!['Z', 'S', 'N', 'P', 'H', 'T']);
    stacks.push(vec!['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D']);
    stacks.push(vec!['P', 'M', 'G', 'F', 'W', 'D', 'Z']);
    stacks.push(vec!['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P']);
    stacks.push(vec!['N', 'D', 'S']);
    stacks.push(vec!['D', 'Z', 'S', 'F', 'M']);

    content
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| -> Move {
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
        })
        .for_each(|mv| {
            for _ in 0..mv.quantity {
                let item = stacks[mv.source].pop().unwrap();
                stacks[mv.dest].push(item);
            }
        });

    let message = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("Answer (part 1): {message}");
}

#[derive(Debug)]
struct Move {
    quantity: u32,
    source: usize,
    dest: usize,
}
