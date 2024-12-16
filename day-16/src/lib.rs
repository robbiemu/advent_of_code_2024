use game_grid::{Grid, GridCell, GridPosition, ParseCellError};
#[cfg(feature = "part2")]
use std::collections::HashSet;


const DATA: &str = include_str!("../input.txt");
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // y, x

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
  Forward,
  Left,
  Right,
}

impl Action {
  pub fn get_cost(&self) -> u32 {
    match self {
      Action::Forward => 1,
      Action::Left | Action::Right => 1000,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct State {
  position: Point,
  orientation: (i32, i32),
}

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
pub struct Consequent {
  #[cfg(not(feature = "part2"))]
  pub path: Vec<Point>,

  #[cfg(feature = "part2")]
  pub locations: HashSet<Point>,
  pub total_cost: u32,
}


fn rotate_left(direction: (i32, i32)) -> (i32, i32) {
  match DIRECTIONS.iter().position(|&d| d == direction) {
    Some(index) => {
      DIRECTIONS[(index + DIRECTIONS.len() - 1) % DIRECTIONS.len()]
    }
    None => unreachable!("Invalid direction provided to rotate_left"),
  }
}

fn rotate_right(direction: (i32, i32)) -> (i32, i32) {
  match DIRECTIONS.iter().position(|&d| d == direction) {
    Some(index) => DIRECTIONS[(index + 1) % DIRECTIONS.len()],
    None => unreachable!("Invalid direction provided to rotate_right"),
  }
}

pub fn get_successors(
  state: &State,
  maze: &ProblemDefinition,
) -> Vec<(State, u32)> {
  let mut successors = Vec::new();
  let position = state.position;
  let current_dir = state.orientation;

  for action in &[Action::Forward, Action::Left, Action::Right] {
    match action {
      Action::Forward => {
        let (dy, dx) = current_dir;
        let new_x = position.x + dx;
        let new_y = position.y + dy;
        let new_point = Point { x: new_x, y: new_y };

        if maze.is_in_bounds(new_point)
          && (maze[new_point] == Cell::Empty || maze[new_point] == Cell::Goal)
        {
          let new_state =
            State { position: new_point, orientation: current_dir };
          successors.push((new_state, action.get_cost()));
        }
      }
      Action::Left => {
        let new_dir = rotate_left(current_dir);
        let new_state = State { position, orientation: new_dir };
        successors.push((new_state, action.get_cost()));
      }
      Action::Right => {
        let new_dir = rotate_right(current_dir);
        let new_state = State { position, orientation: new_dir };
        successors.push((new_state, action.get_cost()));
      }
    }
  }

  successors
}

fn heuristic(state: &State, goal: &Point) -> u32 {
  ((state.position.x - goal.x).abs() + (state.position.y - goal.y).abs()) as u32
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
  #[cfg(not(feature = "part2"))]
  use pathfinding::prelude::astar;
  #[cfg(feature = "part2")]
  use pathfinding::prelude::astar_bag;

  use crate::{
    get_successors, heuristic, src_provider, Cell, Consequent, Point,
    ProblemDefinition, State,
  };


  pub fn extract() -> Result<ProblemDefinition, String> {
    src_provider()?
      .trim()
      .parse()
      .map_err(|_| "Error parsing grid".into())
  }

  pub fn transform(mut data: ProblemDefinition) -> Result<Consequent, String> {
    let start_pos = data
      .iter::<Point>()
      .find_map(
        |(pos, cell)| if cell == Cell::Start { Some(pos) } else { None },
      )
      .ok_or_else(|| "Start cell not found".to_string())?;
    data.set_cell(start_pos, Cell::Empty);

    let goal = data
      .iter::<Point>()
      .find_map(|(pos, cell)| if cell == Cell::Goal { Some(pos) } else { None })
      .ok_or_else(|| "Goal cell not found".to_string())?;

    let initial_state = State { position: start_pos, orientation: (0, 1) };

    #[cfg(feature = "part2")]
    {
      let result = astar_bag(
        &initial_state,
        |state| get_successors(state, &data),
        |state| heuristic(state, &goal),
        |state| state.position == goal,
      );

      match result {
        Some((paths, cost)) => {
          // Aggregate unique locations from all equivalent-cost paths
          let locations = paths
            .into_iter()
            .flat_map(|path| path.into_iter().map(|state| state.position))
            .collect();

          Ok(Consequent { locations, total_cost: cost })
        }
        None => Err("No path found".to_string()),
      }
    }

    #[cfg(not(feature = "part2"))]
    {
      let result = astar(
        &initial_state,
        |state| get_successors(state, &data), // Pass the full `State`
        |state| heuristic(state, &goal),
        |state| state.position == goal,
      );

      match result {
        Some((path, cost)) => {
          let path_points =
            path.into_iter().map(|state| state.position).collect();
          Ok(Consequent { path: path_points, total_cost: cost })
        }
        None => Err("No path found".to_string()),
      }
    }
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    if let Ok(consequent) = result {
      println!("Total Cost: {}", consequent.total_cost);

      #[cfg(not(feature = "part2"))]
      {
        println!("Path: {:?}", consequent.path);
      }

      #[cfg(feature = "part2")]
      {
        println!("Number of Paths: {}", consequent.locations.len());
      }

      Ok(())
    } else {
      Err(result.err().unwrap())
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
  #[test]
  #[mry::lock(src_provider)]
  fn test_transform() {
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));

    let data = extract().expect("failed to extract data");
    let result = transform(data).expect("failed to transform data");

    #[cfg(not(feature = "part2"))]
    {
      assert_eq!(result.total_cost, 7036);
    }

    #[cfg(feature = "part2")]
    {
      assert_eq!(result.locations.len(), 45);
    }
  }

  #[test]
  #[mry::lock(src_provider)]
  #[cfg(feature = "part2")]
  fn test_transform_part2() {
    mock_src_provider()
      .returns(Ok(include_str!("../sample.2.txt").to_string()));

    let data = extract().expect("failed to extract data");
    let result = transform(data).expect("failed to transform data");

    assert_eq!(result.locations.len(), 64);
  }

  // MARK load
}
