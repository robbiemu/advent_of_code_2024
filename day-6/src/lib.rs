use game_grid::{Grid, GridCell, GridPosition, ParseCellError};
use std::{collections::HashSet, hash::Hash};


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

const HAS_LOOP: bool = true;

#[derive(GridCell, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Cell {
  #[cell('.')]
  #[default]
  Empty,
  #[cell('#')]
  Obstacle,
  #[cell('^')]
  Guard,
}

#[derive(GridPosition, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  Left,
  Up,
  Right,
  Down,
}

impl Direction {
  pub fn move_point(&self, point: Point) -> Point {
    match self {
      Self::Left => Point::new(point.x - 1, point.y),
      Self::Up => Point::new(point.x, point.y - 1),
      Self::Right => Point::new(point.x + 1, point.y),
      Self::Down => Point::new(point.x, point.y + 1),
    }
  }
}

pub type ProblemDefinition = Grid<Cell>;
pub type Consequent = i16;


fn cells_in_direction_until_obstacle(
  grid: &Grid<Cell>,
  guard_position: Point,
  direction: Direction,
) -> Vec<(Point, &Cell)> {
  let mut cells = Vec::new();
  let mut current_position = guard_position;

  loop {
    let next_position = direction.move_point(current_position);
    if !grid.is_in_bounds(next_position.clone()) {
      break;
    }

    let cell = &grid[next_position.clone()];
    if *cell == Cell::Obstacle {
      break;
    }

    current_position = next_position.clone();
    cells.push((next_position.to_owned(), cell));
  }

  cells
}

fn simulate_guard_movement(
  grid: &ProblemDefinition,
  start_position: Point,
) -> (HashSet<Point>, bool) {
  let directions = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
  ];
  let mut visited_positions: HashSet<Point> = HashSet::new();
  let mut loop_track: HashSet<Point> = HashSet::new();
  let mut current_position: Point = start_position;

  let mut direction_index = 0;
  visited_positions.insert(current_position.clone());

  loop {
    let current_direction = directions[direction_index];
    let cells = cells_in_direction_until_obstacle(
      grid,
      current_position.clone(),
      current_direction,
    );

    // Determine the last position reached
    let last_position = cells
      .last()
      .map(|(pos, _)| pos.clone())
      .unwrap_or(current_position.clone());

    // Check if the next position beyond the last is out of bounds or an obstacle
    let beyond_next_position =
      current_direction.move_point(last_position.clone());

    for pos in cells.iter().map(|(p, _)| p.to_owned()) {
      if visited_positions.contains(&pos) {
        if loop_track.contains(&pos) {
          return (visited_positions, HAS_LOOP);
        }
        loop_track.insert(pos);
      } else {
        loop_track.clear();
      }
    }

    // Add all traversed cells to visited
    visited_positions.extend(cells.iter().map(|(pos, _)| pos.clone()));

    if !grid.is_in_bounds(beyond_next_position) {
      break; // Stop if we reached the edge or an obstacle
    } else {
      direction_index = (direction_index + 1) % directions.len();
    }

    // Otherwise, update current position and continue
    current_position = last_position;
  }

  (visited_positions, !HAS_LOOP)
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
    simulate_guard_movement, src_provider, Cell, Consequent, Point,
    ProblemDefinition,
  };


  pub fn extract() -> Result<ProblemDefinition, String> {
    src_provider()?
      .parse()
      .map_err(|_| "Error parsing grid".into())
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let guard_position = data
      .iter::<Point>()
      .find(|(_, cell)| *cell == Cell::Guard)
      .map(|(pos, _)| pos)
      .ok_or_else(|| "No guard found".to_string())?;
    #[cfg(not(feature = "part2"))]
    {
      let (visited_positions, _) =
        simulate_guard_movement(&data, guard_position);

      Ok(visited_positions.len() as Consequent)
    }
    #[cfg(feature = "part2")]
    {
      let (initial_visited_positions, _) =
        simulate_guard_movement(&data, guard_position.clone());

      let mut loop_causing_obstacles_count = 0;

      // Test each visited position for loop-causing obstacles
      initial_visited_positions
        .iter()
        .for_each(|visited_position| {
          if *visited_position != guard_position {
            // Create a new grid with an obstacle at the visited position
            let mut modified_data = data.clone();
            modified_data.set_cell(visited_position.clone(), Cell::Obstacle);

            // Simulate with the new grid
            let (_, loop_detected) =
              simulate_guard_movement(&modified_data, guard_position.clone());
            if loop_detected {
              loop_causing_obstacles_count += 1;
            }
          }
        });

      Ok(loop_causing_obstacles_count)
    }
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(c) => {
        println!("{c} spaces visited");

        Ok(())
      }
      Err(e) => Err(e),
    }
  }
}


#[cfg(test)]
mod tests {
  use super::{prelude::*, *};

  // MARK extract
  #[test]
  #[mry::lock(src_provider)] // Lock the function for mocking.
  fn test_extract() {
    let data = "#.
^.";
    mock_src_provider().returns(Ok(data.to_string()));
    let result = extract().expect("Failed to extract data");

    assert_eq!(
      result.iter().collect::<Vec<(Point, Cell)>>(),
      vec![
        (Point::new(0, 0), Cell::Obstacle),
        (Point::new(1, 0), Cell::Empty),
        (Point::new(0, 1), Cell::Guard),
        (Point::new(1, 1), Cell::Empty),
      ]
    );
  }

  // MARK transform
  #[cfg(all(feature = "sample", not(feature = "part2")))]
  #[test]
  fn test_transform() {
    let data = extract().expect("Failed to extract data");
    let result = transform(data);

    assert_eq!(result, Ok(41));
  }
  #[cfg(all(feature = "sample", feature = "part2"))]
  #[test]
  fn test_transform() {
    let data = extract().expect("Failed to extract data");
    let result = transform(data);

    assert_eq!(result, Ok(6));
  }

  // MARK load
}
