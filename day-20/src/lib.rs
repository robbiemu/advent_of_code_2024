use game_grid::{Grid, GridCell, GridPosition, ParseCellError};
use itertools::Itertools;
use pathfinding::prelude::bfs;


const DATA: &str = include_str!("../input.txt");
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // y, x
const MIN_CHEAT: usize = 100;
#[cfg(not(feature = "part2"))]
const CHEAT_LENGTH: usize = 2;
#[cfg(feature = "part2")]
const CHEAT_LENGTH: usize = 20;

#[derive(GridCell, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Cell {
  #[cell('.')]
  #[default]
  Empty,
  #[cell('#')]
  Wall,
  #[cell('S')]
  Start,
  #[cell('E')]
  Goal,
}

#[derive(GridPosition, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
  x: i32,
  y: i32,
}

impl std::fmt::Debug for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "(x{},y{})", self.x, self.y)
  }
}

pub type ProblemDefinition = Grid<Cell>;
pub type Consequent = usize;


fn get_key_points(grid: &Grid<Cell>) -> Result<Vec<Point>, String> {
  let mut start: Option<Point> = None;
  let mut end: Option<Point> = None;

  for (position, cell) in grid.iter() {
    match cell {
      Cell::Start => start = Some(position),
      Cell::Goal => end = Some(position),
      _ => (),
    }
  }

  let Some(start_pos) = start else {
    return Err("No start found".to_string());
  };
  let Some(end_pos) = end else {
    return Err("No end found".to_string());
  };

  let Some(normal_path) =
    bfs(&start_pos, |&p| successors(p, grid), |&p| p == end_pos)
  else {
    return Err("No path found".to_string());
  };

  Ok(normal_path)
}

fn successors(p: Point, grid: &Grid<Cell>) -> Vec<Point> {
  DIRECTIONS
    .iter()
    .filter_map(|(dy, dx)| {
      let new_position = Point { x: p.x + dx, y: p.y + dy };
      if grid.is_in_bounds(new_position)
        && matches!(grid[new_position], Cell::Empty | Cell::Goal)
      {
        Some(new_position)
      } else {
        None
      }
    })
    .collect()
}

fn manhattan_distance(a: &Point, b: &Point) -> usize {
  ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize
}

fn find_all_shorter_paths(
  grid: &Grid<Cell>,
) -> Result<(Vec<usize>, usize), String> {
  let normal_path = get_key_points(grid)?;

  let mut cheat_paths = Vec::new();

  let normal_distance_map = normal_path
    .iter()
    .enumerate()
    .map(|(i, p)| (*p, i))
    .collect::<std::collections::HashMap<Point, usize>>();
  let normal_cost = normal_path.len();

  for (left, right) in normal_path.iter().tuple_combinations() {
    let d = manhattan_distance(left, right);
    if d <= CHEAT_LENGTH {
      let (start, end) =
        if normal_distance_map[left] < normal_distance_map[right] {
          (normal_distance_map[left], normal_distance_map[right])
        } else {
          (normal_distance_map[right], normal_distance_map[left])
        };
      cheat_paths.push(start + d + (normal_cost - end));
    }
  }

  if !cheat_paths.is_empty() {
    Ok((cheat_paths, normal_path.len()))
  } else {
    Err("No shorter paths found".to_string())
  }
}

#[cfg(test)]
#[mry::mry]
fn get_min_cheat() -> usize {
  MIN_CHEAT
}
#[cfg(not(test))]
fn get_min_cheat() -> usize {
  MIN_CHEAT
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
  use crate::{
    find_all_shorter_paths, get_min_cheat, src_provider, Consequent,
    ProblemDefinition,
  };

  pub fn extract() -> Result<ProblemDefinition, String> {
    src_provider()?
      .trim()
      .parse()
      .map_err(|_| "Error parsing grid".into())
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let (cheat_paths, normal_cost) = find_all_shorter_paths(&data)?;
    let min_cheat = get_min_cheat();

    Ok(
      cheat_paths
        .iter()
        .filter_map(|cheat_path| {
          if cheat_path + min_cheat <= normal_cost {
            Some(())
          } else {
            None
          }
        })
        .count(),
    )
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(paths) => {
        println!("Path length: {}", paths);

        Ok(())
      }
      Err(e) => Err(e),
    }
  }
}


#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::{prelude::*, *};

  // MARK extract
  // #[mry::lock(src_provider)] // Lock the function for mocking.

  // MARK transform
  #[cfg(not(feature = "part2"))]
  #[test]
  #[mry::lock(get_min_cheat)]
  #[mry::lock(src_provider)]
  fn test_transform() {
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));
    mock_get_min_cheat().returns(1);

    let data = extract().expect("failed to extract data");
    let result = transform(data);

    assert_eq!(result, Ok(44));
  }

  #[cfg(feature = "part2")]
  #[test]
  #[mry::lock(get_min_cheat)]
  #[mry::lock(src_provider)]
  fn test_transform() {
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));
    mock_get_min_cheat().returns(50);

    let data = extract().expect("failed to extract data");
    let result = transform(data);

    assert_eq!(
      result,
      Ok(32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3)
    );
  }

  // MARK load
}
