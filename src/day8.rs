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
     
      let mut vleft = true;
 
      for prev in 0..col {
        let hp = heights.get(&(row, prev)).unwrap();
        if h < hp {
          vleft = false;
          break;
        } 
      }
 
      if vleft {
        visible.insert((row, col));
      }
      
      let mut vtop = true;
      
      for prev in 0..row {
        let hp = heights.get(&(prev, col)).unwrap();
        if h < hp {
          vright = false;
          break;
        }
      }
      
      if vtop {
        visible.insert((row, col));
      } 
    }
  }
  
  for row in (0..n).rev() {
    for col in (0..n).rev() {
      let h = heights.get(&(row, col)).unwrap();
     
      let mut vright = true;
 
      for prev in (col + 1..n).rev() {
        let hp = heights.get(&(row, prev)).unwrap();
        if h < hp {
          vright = false;
          break;
        } 
      }
 
      if vright {
        visible.insert((row, col));
      }
      
      let mut vbottom = true;
      
      for prev in (row + 1..n).rev() {
        let hp = heights.get(&(prev, col)).unwrap();
        if h < hp {
          vbottom = false;
          break;
        }
      }
      
      if vbottom {
        visible.insert((row, col));
      }
    }
  } 
  
  return visible.len() as u32;
}