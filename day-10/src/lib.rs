use game_grid::{Grid, GridCell, GridPosition, ParseCellError};
use std::collections::{HashMap, HashSet, VecDeque};


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

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
      Cell::Trail(c) => (*c as u8) - b'1' + 1,
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
  fn neighbors<'a>(
    &'a self,
    point: &'a Point,
  ) -> impl Iterator<Item = Point> + 'a {
    DIRECTIONS.iter().filter_map(move |(dx, dy)| {
      let p = Point { x: point.x + dx, y: point.y + dy };
      self.0.is_in_bounds(p).then_some(p)
    })
  }
}


#[derive(Debug, Clone)]
struct PathState {
  path: Vec<Point>,
  indices: HashSet<u8>,
  #[cfg(not(feature = "part2"))]
  step: u8,
}

#[cfg(not(feature = "part2"))]
type ResultSet = HashSet<Point>;
#[cfg(feature = "part2")]
type ResultSet = HashSet<Vec<Point>>;


fn multi_source_bfs(
  mut visited: HashMap<Point, PathState>,
  mut queue: VecDeque<PathState>,
  grid: &WrappedGrid,
) -> Vec<ResultSet> {
  let mut result: Vec<ResultSet> = vec![ResultSet::new(); visited.len()];

  while let Some(state) = queue.pop_front() {
    let &last_point = state.path.last().unwrap();
    let last_cell = grid.0[last_point];
    if matches!(last_cell, Cell::Destiny) {
      for &idx in &state.indices {
        #[cfg(not(feature = "part2"))]
        result[idx as usize].insert(last_point);
        #[cfg(feature = "part2")]
        result[idx as usize].insert(state.path.clone());
      }
      continue;
    }

    let new_step = last_cell.cell_value() + 1;
    for neighbor in grid.neighbors(&last_point) {
      let neighbor_val = grid.0[neighbor].cell_value();
      if neighbor_val == new_step {
        #[cfg(not(feature = "part2"))]
        {
          if let Some(existing) = visited.get_mut(&neighbor) {
            existing.indices.extend(&state.indices);
            existing.step = new_step;
            existing.path.push(neighbor);
            queue.push_back(existing.clone());
            continue;
          }
          let mut new_path = state.path.clone();
          new_path.push(neighbor);
          let new_state = PathState {
            path: new_path,
            step: new_step,
            indices: state.indices.clone(),
          };
          visited.insert(neighbor, new_state.clone());
          queue.push_back(new_state);
        }

        #[cfg(feature = "part2")]
        {
          let mut new_path = state.path.clone();
          new_path.push(neighbor);
          let new_state =
            PathState { path: new_path, indices: state.indices.clone() };
          visited.entry(neighbor).or_insert(new_state.clone());
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
  use super::*;

  pub fn extract() -> Result<ProblemDefinition, String> {
    src_provider()?
      .trim()
      .parse()
      .map_err(|_| "Error parsing grid".into())
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let mut visited: HashMap<Point, PathState> = HashMap::new();
    let mut queue = VecDeque::new();

    // Initialize visited and queue during grid traversal
    data.iter::<Point>().for_each(|(p, c)| {
      if matches!(c, Cell::Trailhead) {
        let idx = visited.len() as u8;
        let state = PathState {
          path: vec![p],
          indices: HashSet::from([idx]),
          #[cfg(not(feature = "part2"))]
          step: 0,
        };
        visited.insert(p, state.clone());
        queue.push_back(state);
      }
    });

    Ok(
      multi_source_bfs(visited, queue, &WrappedGrid(data))
        .into_iter()
        .map(|positions| positions.len() as u32)
        .collect(),
    )
  }


  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(consequent) => {
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

  #[cfg(all(feature = "sample", not(feature = "part2")))]
  #[test]
  fn test_transform() {
    let data = extract().expect("Failed to extract data");
    assert_eq!(transform(data), Ok(vec![5_u32, 6, 5, 3, 1, 3, 5, 3, 5]));
  }

  #[cfg(all(feature = "sample", feature = "part2"))]
  #[test]
  fn test_transform() {
    let data = extract().expect("Failed to extract data");
    assert_eq!(transform(data), Ok(vec![20_u32, 24, 10, 4, 1, 4, 5, 8, 5]));
  }
}
