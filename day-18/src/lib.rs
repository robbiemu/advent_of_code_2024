use glam::IVec2;


const DATA: &str = include_str!("../input.txt");
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // y, x
const GOAL: IVec2 = IVec2::new(70, 70);
#[cfg(not(feature = "part2"))]
const SNAPSHOT: usize = 1024;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cell {
  Empty,
  Wall,
}

pub struct Grid {
  cells: Vec<Cell>,
  width: i32,
  height: i32,
}

impl Grid {
  pub fn new(size_point: IVec2) -> Self {
    let width = size_point.x + 1;
    let height = size_point.y + 1;
    let cells = vec![Cell::Empty; (width * height) as usize];
    Grid { cells, width, height }
  }

  pub fn is_in_bounds(&self, pos: IVec2) -> bool {
    pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
  }
  pub fn at(&self, pos: IVec2) -> Cell {
    self.cells[(pos.y * self.width + pos.x) as usize]
  }
  pub fn set(&mut self, pos: IVec2, cell: Cell) {
    self.cells[(pos.y * self.width + pos.x) as usize] = cell;
  }
}

#[derive(Debug, Default)]
pub struct ProblemDefinition {
  obstacles: Vec<IVec2>,
  goal: IVec2,
}
#[cfg(not(feature = "part2"))]
pub type Consequent = Vec<IVec2>;
#[cfg(feature = "part2")]
pub type Consequent = IVec2;


pub fn get_successors(state: IVec2, maze: &Grid) -> Vec<(IVec2, u32)> {
  let mut successors = Vec::new();

  for action in &DIRECTIONS {
    let (dy, dx) = action;
    let new_x = state.x + dx;
    let new_y = state.y + dy;
    let new_point = IVec2::new(new_x, new_y);
    if maze.is_in_bounds(new_point) && maze.at(new_point) == Cell::Empty {
      successors.push((new_point, 1));
    }
  }

  successors
}

fn heuristic(state: IVec2, goal: IVec2) -> u32 {
  ((state.x - goal.x).abs() + (state.y - goal.y).abs()) as u32
}

#[cfg(test)]
#[mry::mry]
fn get_goal() -> IVec2 {
  GOAL
}
#[cfg(not(test))]
fn get_goal() -> IVec2 {
  GOAL
}

#[cfg(all(test, not(feature = "part2")))]
#[mry::mry]
fn get_snapshot() -> usize {
  SNAPSHOT
}
#[cfg(all(not(test), not(feature = "part2")))]
fn get_snapshot() -> usize {
  SNAPSHOT
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
  use glam::IVec2;
  use pathfinding::prelude::astar;

  #[cfg(not(feature = "part2"))]
  use crate::get_snapshot;
  use crate::{
    get_goal, get_successors, heuristic, src_provider, Cell, Consequent, Grid,
    ProblemDefinition,
  };


  pub fn extract() -> Result<ProblemDefinition, String> {
    let mut pd = ProblemDefinition::default();

    for line in src_provider()?.trim().lines() {
      let Some((x_str, y_str)) = line.split_once(',') else {
        return Err(format!(
          "error processing obstacle {line}: no comma found"
        ));
      };

      let x = x_str
        .parse::<i32>()
        .map_err(|e| format!("error parsing x coordinate in {line}: {e}"))?;
      let y = y_str
        .parse::<i32>()
        .map_err(|e| format!("error parsing y coordinate in {line}: {e}"))?;

      let obstacle = IVec2 { x, y };
      pd.obstacles.push(obstacle);
    }

    pd.goal = get_goal();

    Ok(pd)
  }

  #[cfg(not(feature = "part2"))]
  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let snapshot = get_snapshot();
    let goal = get_goal();

    let mut grid = Grid::new(goal);
    data
      .obstacles
      .iter()
      .take(snapshot)
      .for_each(|obstacle| grid.set(*obstacle, Cell::Wall));

    let result = astar(
      &IVec2::new(0, 0),
      |state| get_successors(*state, &grid), // Pass the full `State`
      |state| heuristic(*state, goal),
      |state| *state == goal,
    );

    match result {
      Some((path, _cost)) => Ok(path),
      None => Err("No path found".to_string()),
    }
  }
  #[cfg(feature = "part2")]
  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let goal = get_goal();

    let mut grid = Grid::new(goal);
    let mut result = None;
    for i in 0..data.obstacles.len() {
      data
        .obstacles
        .iter()
        .take(i + 1)
        .for_each(|obstacle| grid.set(*obstacle, Cell::Wall));

      let run_result = astar(
        &IVec2::new(0, 0),
        |state| get_successors(*state, &grid), // Pass the full `State`
        |state| heuristic(*state, goal),
        |state| *state == goal,
      );
      if run_result.is_none() {
        result = Some((data.obstacles[i], i));
        break;
      }
    }

    match result {
      Some((location, _step)) => Ok(location),
      None => Err("No path found".to_string()),
    }
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(output) => {
        #[cfg(not(feature = "part2"))]
        println!("Path to goal: {:?}\n(len {})", output, output.len() - 1);
        #[cfg(feature = "part2")]
        println!("first closing obstacle: {:?}", output);

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
  #[cfg(all(not(feature = "part2"), not(feature = "bench")))]
  #[test]
  #[mry::lock(src_provider)]
  #[mry::lock(get_goal)]
  #[mry::lock(get_snapshot)]
  fn test_transform() {
    mock_get_snapshot().returns(12);
    mock_get_goal().returns(IVec2::new(6, 6));
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));

    let data = extract().expect("failed to extract data");
    let result = transform(data).expect("failed to transform data");
    assert_eq!(result.len() - 1, 22);
  }

  #[cfg(feature = "part2")]
  #[test]
  #[mry::lock(src_provider)]
  #[mry::lock(get_goal)]
  fn test_transform() {
    mock_get_goal().returns(IVec2::new(6, 6));
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));

    let data = extract().expect("failed to extract data");
    let result = transform(data).expect("failed to transform data");
    assert_eq!(result, IVec2::new(6, 1));
  }

  // MARK load
}
