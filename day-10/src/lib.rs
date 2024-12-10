use game_grid::{Grid, GridCell, GridPosition, ParseCellError};
use std::collections::{HashMap, HashSet, VecDeque};


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

const DIRECTIONS: [(i32, i32); 4] = [
  (0, 1),  // right
  (0, -1), // left
  (1, 0),  // down
  (-1, 0), // up
];


pub type ProblemDefinition = Grid<Cell>;
pub type Consequent = Vec<u32>;

#[derive(GridCell, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Cell {
  #[cell('0')]
  #[default]
  Trailhead,
  #[cell('1'..='8')]
  Trail(char),
  #[cell('9')]
  Destiny,
}

impl Cell {
  fn cell_value(&self) -> u8 {
    match self {
      Cell::Trailhead => 0,
      Cell::Trail(c) => (*c) as u8 - b'1' + 1,
      Cell::Destiny => 9,
    }
  }
}

#[derive(
  GridPosition, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

pub struct WrappedGrid(Grid<Cell>);

impl WrappedGrid {
  fn get_directions(&self, point: &Point) -> [Option<Point>; 4] {
    let mut directions = [None; 4];
    for (i, &dir) in DIRECTIONS.iter().enumerate() {
      let destination = Point { x: point.x + dir.0, y: point.y + dir.1 };
      if self.0.is_in_bounds(destination) {
        directions[i] = Some(destination);
      }
    }

    directions
  }
}

#[derive(Debug, Clone)]
struct PathState {
  path: Vec<Point>,
  step: u8,
  indices: HashSet<u8>,
}


fn multi_source_bfs(
  starts: &[Point],
  goals: &HashSet<Point>,
  grid: &WrappedGrid,
) -> Vec<HashSet<Point>> {
  let mut result: Vec<HashSet<Point>> = vec![HashSet::new(); starts.len()];
  let mut visited: HashMap<Point, PathState> = HashMap::new();
  let mut queue = VecDeque::new();

  // Initialize with all starting points
  for (idx, &start) in starts.iter().enumerate() {
    let initial_state = PathState {
      path: vec![start],
      step: 0,
      indices: HashSet::from([idx as u8]),
    };
    visited.insert(start, initial_state.clone());
    queue.push_back(initial_state);
  }

  while let Some(state) = queue.pop_front() {
    let last_point = state.path.last().unwrap();
    let last_cell = grid.0[*last_point];

    // Check for goal
    if goals.contains(last_point) {
      state.indices.iter().for_each(|&idx| {
        result[idx as usize].insert(*last_point);
      });
      continue;
    }

    // Expand neighbors
    for neighbor in grid.get_directions(last_point).iter().flatten() {
      let neighbor_cell = grid.0[*neighbor];

      // Movement "up" by 1
      let new_step = last_cell.cell_value() + 1;
      if neighbor_cell.cell_value() == new_step {
        if let Some(existing_state) = visited.get_mut(neighbor) {
          // Merge indices if we're at the same step and point
          existing_state.indices.extend(state.indices.clone());
          existing_state.step = new_step;
          existing_state.path.push(*neighbor);
          queue.push_back(existing_state.clone());
        } else {
          // Create a new state and add it
          let mut new_path = state.path.clone();
          new_path.push(*neighbor);
          let new_state = PathState {
            path: new_path,
            step: new_step,
            indices: state.indices.clone(),
          };
          visited.insert(*neighbor, new_state.clone());
          queue.push_back(new_state);
        }
      }
    }
  }

  result
}

#[cfg(test)]
#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}
#[cfg(not(test))]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

pub mod prelude {
  use std::collections::HashSet;

  use crate::{
    multi_source_bfs, src_provider, Cell, Consequent, Point, ProblemDefinition,
    WrappedGrid,
  };


  pub fn extract() -> Result<ProblemDefinition, String> {
    src_provider()?
      .parse()
      .map_err(|_| "Error parsing grid".into())
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let (zeros, nines) = data.iter::<Point>().fold(
      (Vec::new(), HashSet::new()),
      |(mut zeros, mut nines), (point, cell)| match cell {
        Cell::Trailhead => {
          zeros.push(point);

          (zeros, nines)
        }
        Cell::Destiny => {
          nines.insert(point);

          (zeros, nines)
        }
        _ => (zeros, nines),
      },
    );

    Ok(
      multi_source_bfs(&zeros, &nines, &WrappedGrid(data))
        .into_iter()
        .map(|positions| positions.len() as u32)
        .collect(),
    )
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(consequent) => {
        dbg!(&consequent);
        println!("Consequent: {:?}", consequent.iter().sum::<u32>());

        Ok(())
      }
      Err(e) => Err(e),
    }
  }
}


#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::prelude::*;

  // MARK extract
  // #[mry::lock(src_provider)] // Lock the function for mocking.

  // MARK transform
  #[cfg(all(feature = "sample", not(feature = "part2")))]
  #[test]
  fn test_transform() {
    let data = extract().expect("Failed to extract data");
    assert_eq!(transform(data), Ok(vec![5_u32, 6, 5, 3, 1, 3, 5, 3, 5]));
  }

  // MARK load
}
