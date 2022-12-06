use std::collections::HashSet;

pub fn run() {
    let content = include_str!("inputs/6.txt");

    let num_processed = find_marker_start(4, content);
    println!("Answer (part 1): {num_processed}");

    let num_processed = find_marker_start(14, content);
    println!("Answer (part 2): {num_processed}");
}

fn find_marker_start(size: usize, content: &str) -> usize {
    let mut chars = content.chars();
    let mut window = Vec::new();
    let mut num_processed = 0;

    loop {
        let c = chars.next().unwrap();
        num_processed += 1;
        window.push(c);

        if num_processed <= size {
            continue;
        }

        window.remove(0);
        assert!(window.len() == size);

        let unique_chars: HashSet<char> = window.clone().into_iter().collect();

        if window.len() == unique_chars.len() {
            return num_processed;
        }
    }
}
