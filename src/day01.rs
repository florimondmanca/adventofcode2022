use itertools::sorted;

pub fn run() {
    let content = include_str!("inputs/01.txt");

    let mut elves = Vec::new();

    {
        let mut calories = 0;

        for line in content.lines() {
            if line.is_empty() {
                elves.push(calories);
                calories = 0;
            } else {
                calories += line.parse::<i32>().unwrap();
            }
        }
    }

    let max_calories = elves.iter().max().unwrap();
    println!("Answer (part 1): {}", max_calories);

    let total_top3_calories = sorted(elves).rev().take(3).sum::<i32>();
    println!("Answer (part 2): {}", total_top3_calories);
}
