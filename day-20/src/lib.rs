use game_grid::{Grid, GridCell, GridPosition, ParseCellError};
use pathfinding::prelude::bfs;

const DATA: &str = include_str!("../input.txt");
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // y, x
const MIN_CHEAT: usize = 100;

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


fn get_key_points(
  grid: &Grid<Cell>,
) -> Result<(Point, Point, Vec<Point>, Vec<Point>), String> {
  let mut start: Option<Point> = None;
  let mut end: Option<Point> = None;
  let mut interesting_walls = Vec::new();

  for (position, cell) in grid.iter() {
    match cell {
      Cell::Start => start = Some(position),
      Cell::Goal => end = Some(position),
      Cell::Wall => {
        let count = DIRECTIONS.iter().fold(0, |acc, (dy, dx)| {
          let new_position = Point { x: position.x + dx, y: position.y + dy };
          if grid.is_in_bounds(new_position)
            && matches!(
              grid[new_position],
              Cell::Empty | Cell::Goal | Cell::Start
            )
          {
            acc + 1
          } else {
            acc
          }
        });
        if count > 1 {
          interesting_walls.push(position);
        }
      }
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

  Ok((start_pos, end_pos, interesting_walls, normal_path))
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

fn find_all_shorter_paths(
  grid: &Grid<Cell>,
) -> Result<(Vec<Vec<Point>>, usize), String> {
  let (_start, _end, interesting_walls, normal_path) = get_key_points(grid)?;

  let mut cheat_paths = Vec::new();
  for wall in interesting_walls {
    let Some(points) = get_path_neighbors(wall, &normal_path) else {
      return Err("No neighbors of interesting wall found".to_string());
    };

    for (i, &start_point) in points.iter().enumerate() {
      for &end_point in points.iter().skip(i + 1) {
        let start_idx =
          normal_path.iter().position(|&p| p == start_point).unwrap();
        let end_idx = normal_path.iter().position(|&p| p == end_point).unwrap();

        let (start_idx, end_idx) = if start_idx < end_idx {
          (start_idx, end_idx)
        } else {
          (end_idx, start_idx)
        };

        if end_idx - start_idx > 2 {
          let mut cheat_path = normal_path
            .iter()
            .take(start_idx + 1)
            .copied()
            .collect::<Vec<Point>>();

          cheat_path.push(wall);

          cheat_path.extend(normal_path.iter().skip(end_idx).copied());

          cheat_paths.push(cheat_path);
        }
      }
    }
  }

  if !cheat_paths.is_empty() {
    return Ok((cheat_paths, normal_path.len()));
  }

  Err("No shorter paths found".to_string())
}

fn get_path_neighbors(point: Point, path: &[Point]) -> Option<Vec<Point>> {
  let mut neighbors = Vec::new();

  for (dy, dx) in DIRECTIONS.iter() {
    let new_position = Point { x: point.x + dx, y: point.y + dy };
    if path.contains(&new_position) {
      neighbors.push(new_position);
    }
  }

  if neighbors.len() > 1 {
    Some(neighbors)
  } else {
    None
  }
}

fn print_path_grid(grid: &Grid<Cell>, path: &[Point], cost: usize) {
  let mut display_grid = vec![vec!['.'; grid.width()]; grid.height()];

  for (y, row) in display_grid.iter_mut().enumerate() {
    for (x, cell) in row.iter_mut().enumerate() {
      let pos = Point { x: x as i32, y: y as i32 };
      *cell = match grid[pos] {
        Cell::Wall => '#',
        Cell::Start => 'S',
        Cell::Goal => 'E',
        Cell::Empty => '.',
      };
    }
  }

  for point in path {
    if grid[*point] != Cell::Start && grid[*point] != Cell::Goal {
      display_grid[point.y as usize][point.x as usize] = 'O';
    }
  }

  println!("\nCaminho com custo: {}", cost);
  for row in display_grid {
    println!("{}", row.iter().collect::<String>());
  }
  println!();
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
  //use crate::print_path_grid;
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
          if cheat_path.len() + min_cheat < normal_cost {
            //print_path_grid(&data, cheat_path, cheat_path.len());
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
    mock_get_min_cheat().returns(0);

    let data = extract().expect("failed to extract data");
    let result = transform(data);

    assert_eq!(result, Ok(44));
  }

  // MARK load
}
