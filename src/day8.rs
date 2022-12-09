use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
  let content = include_str!("inputs/8.txt");
  let (heights, n) = parse_heights(content);
  let count = count_visible(heights, n);
  println!("Answer (part 1): {count}");
}

type Heights = HashMap<(usize, usize), u32>;

fn parse_heights(content: &str) -> (Heights, usize) {
  let mut heights: Heights = HashMap::new();
  let n = content.lines().count();
 
  for (i, line) in content.lines().enumerate() {
    line
      .chars()
      .map(|c| c.to_digit(10).unwrap())
      .enumerate()
      .for_each(|(j, h)| {
        heights.insert((i, j), h);
      });
  }
  
  return (heights, n);
}

fn count_visible(heights: Heights, n: usize) -> u32 {
  let mut visible: HashSet<(usize, usize)> = HashSet::new();
  
  for row in 0..n {
    for col in 0..n {
      let h = heights.get(&(row, col)).unwrap();
      let hl = heights.get(&(row, 0)).unwrap();
      let hr = heights.get(&(row, n - 1)).unwrap();
      let ht = heights.get(&(0, col)).unwrap();
      let hb = heights.get(&(n - 1, col)).unwrap();
      if (col > 0 && h < hl) || (col < n - 1 && h < hr) || (row > 0 && h < ht) || (row < n - 1 && h < hb) {
        visible.insert((row, col));
      }
    }
  }
  
  return visible.len() as u32;
}