use std::collections::HashMap;

pub fn run() {
  let content = include_str!("inputs/8.txt");
  let heights = parse_heights(content);
  let count = count_visible(heights);
  println!("Answer (part 1): {count}");
}

type Heights = HashMap<(usize, usize), u32>;

fn parse_heights(content: &str) -> Heights {
  let heights: Heights = HashMap::new();
  let n = content.lines().len();
  
  for (i, line) in content.lines().enumerate() {
    line
      .chars()
      .map(|x| x.parse::<u32>().unwrap())
      .enumerate()
      .for_each(|(j, h)| heights.insert((i, j), h));
  }
  
  return heights;
}

fn count_visible(heights: Heights) -> u32 {
  return 0;
}