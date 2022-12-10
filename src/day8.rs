use std::collections::HashMap;
use std::collections::HashSet;
use itertools::iproduct;

pub fn run() {
  let content = include_str!("inputs/8.txt");
  let grid = parse(content);
  let count = count_visible(grid);
  println!("Answer (part 1): {count}");

  let num_visible = grid
    .cells()
    .map(|(row, col, h)| {
      grid
        .view_from(row, col)
        .into_iter()
        .any(|direction| direction.into_iter().all(|hd| hd < h))
    }) 
    .map(|b| b as u32)
    .sum::<u32>();
  
  println!("Answer (part 1) (alt): {num_visible}");
}

type Heights = HashMap<(usize, usize), u32>;

fn parse(content: &str) -> Grid {
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
  
  return Grid::new(heights, n);
}

struct Grid {
  heights: Heights, 
  size: usize,
}

impl Grid {
  fn new(heights: Heights, size: usize) -> Self {
    Self { heights, size }
  }
  
  fn get(&self, row: usize, col: usize) -> u32 {
    self.heights.get(&(row, col)).unwrap().clone()
  }
  
  fn cells(&self) -> impl Iterator<Item = (usize, usize, u32)> + '_ {
    iproduct!(0..self.size, 0..self.size)
      .map(|(r, c)| (r, c, self.get(r, c)))
  } 
 
  fn view_from(&self, row: usize, col: usize) -> Vec<Vec<u32>> {
    vec![
      // Left
      (0..col - 1)
        .map(|c| self.get(row, c))
        .collect::<Vec<u32>>(), 
      // Right
      (col + 1..self.size)
        .rev() 
        .map(|c| self.get(row, c))
        .collect::<Vec<u32>>(),
      // Down
      (row + 1..self.size)
        .rev()
        .map(|r| self.get(r, col))
        .collect::<Vec<u32>>(),
      // Up
      (0..row - 1)
        .map(|r| self.get(r, col))
        .collect::<Vec<u32>>(),
    ]
  } 
}

fn count_visible(grid: Grid) -> u32 {
  let mut visible: HashSet<(usize, usize)> = HashSet::new();
  
  let n = grid.size;
 
  for row in 0..n {
    for col in 0..n {
      let h = grid.get(row, col);
     
      let mut vleft = true;
 
      for prev in 0..col {
        let hp = grid.get(row, prev);
        if h <= hp {
          vleft = false;
          break;
        } 
      }
 
      if vleft {
        visible.insert((row, col));
        continue;
      }
      
      let mut vtop = true;
      
      for prev in 0..row {
        let hp = grid.get(prev, col);
        if h <= hp {
          vtop = false;
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
      if visible.contains(&(row, col)) {
        continue;
      }
  
      let h = grid.get(row, col);
     
      let mut vright = true;
 
      for prev in col + 1..n {
        let hp = grid.get(row, prev);
        if h <= hp {
          vright = false;
          break;
        }
      }
 
      if vright {
        visible.insert((row, col));
        continue;
      }
      
      let mut vbottom = true;
      
      for prev in row + 1..n {
        let hp = grid.get(prev, col);
        if h <= hp {
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