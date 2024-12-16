use game_grid::{Grid, GridCell, GridPosition, ParseCellError};
use std::cmp::Ordering;
#[cfg(feature = "part2")]
use std::collections::HashSet;
use std::collections::{BinaryHeap, HashMap};


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

#[derive(GridCell, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Cell {
  #[cell('#')]
  #[default]
  Wall,
  #[cell('.')]
  Space,
  #[cell('S')]
  Start,
  #[cell('E')]
  End,
}

#[derive(GridPosition, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl std::fmt::Debug for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "(x{},y{})", self.x, self.y)
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
  position: Point,
  orientation: (i32, i32),
  cost: u32,     // Cumulative cost
  priority: u32, // Cost + heuristic
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    other.priority.cmp(&self.priority) // min-heap
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

pub type PointDirectionPair = (Point, (i32, i32));

pub type ProblemDefinition = Grid<Cell>;

pub struct Consequent {
  #[cfg(not(feature = "part2"))]
  pub path: Vec<Point>,

  #[cfg(feature = "part2")]
  pub locations: HashSet<Point>,

  pub total_cost: u32,
}


fn heuristic(a: (i32, i32), b: (i32, i32)) -> u32 {
  ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

pub fn get_successors(
  position: &Point,
  orientation: &(i32, i32),
  maze: &ProblemDefinition,
) -> Vec<(Point, (i32, i32), u32)> {
  let mut successors = Vec::new();
  let (x, y) = (position.x, position.y);
  let current_dir = *orientation;

  for action in &[Action::Forward, Action::Left, Action::Right] {
    match action {
      Action::Forward => {
        let (dy, dx) = current_dir;
        let new_x = x + dx;
        let new_y = y + dy;
        let new_point = Point { x: new_x, y: new_y };

        if maze.is_in_bounds(new_point)
          && (maze[new_point] == Cell::Space || maze[new_point] == Cell::End)
        {
          successors.push((new_point, current_dir, action.get_cost()));
        }
      }
      Action::Left => {
        let new_dir = rotate_left(current_dir);
        successors.push((*position, new_dir, action.get_cost()));
      }
      Action::Right => {
        let new_dir = rotate_right(current_dir);
        successors.push((*position, new_dir, action.get_cost()));
      }
    }
  }

  successors
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

#[cfg(not(feature = "part2"))]
fn a_star_search(
  start: Point,
  goal: Point,
  start_orientation: (i32, i32),
  maze: &ProblemDefinition,
) -> Option<(Vec<Point>, u32)> {
  let mut open_set = BinaryHeap::new();
  open_set.push(Node {
    position: start,
    orientation: start_orientation,
    cost: 0,
    priority: heuristic((start.x, start.y), (goal.x, goal.y)),
  });

  let mut came_from: HashMap<PointDirectionPair, PointDirectionPair> =
    HashMap::new();
  let mut cost_so_far: HashMap<PointDirectionPair, u32> = HashMap::new();

  came_from.insert((start, start_orientation), (start, start_orientation));
  cost_so_far.insert((start, start_orientation), 0);

  while let Some(current_node) = open_set.pop() {
    let current = current_node.position;
    let current_orientation = current_node.orientation;

    if current == goal {
      let total_cost = cost_so_far[&(current, current_orientation)];
      let mut path = Vec::new();
      let mut current_state = (current, current_orientation);

      while current_state.0 != start || current_state.1 != start_orientation {
        path.push(current_state.0);
        current_state = came_from[&current_state];
      }
      path.push(start);
      path.reverse();

      return Some((path, total_cost));
    }

    let successors = get_successors(&current, &current_orientation, maze);
    for (successor_position, successor_orientation, action_cost) in successors {
      let new_cost = cost_so_far[&(current, current_orientation)] + action_cost;
      let successor_state = (successor_position, successor_orientation);

      if !cost_so_far.contains_key(&successor_state)
        || new_cost < cost_so_far[&successor_state]
      {
        cost_so_far.insert(successor_state, new_cost);
        let priority = new_cost
          + heuristic(
            (successor_position.x, successor_position.y),
            (goal.x, goal.y),
          );
        open_set.push(Node {
          position: successor_position,
          orientation: successor_orientation,
          cost: new_cost,
          priority,
        });
        came_from.insert(successor_state, (current, current_orientation));
      }
    }
  }

  None // No path found
}

#[cfg(feature = "part2")]
fn a_star_search_multi(
  start: Point,
  goal: Point,
  start_orientation: (i32, i32),
  maze: &ProblemDefinition,
) -> Option<(Vec<Vec<Point>>, u32)> {
  let mut open_set = BinaryHeap::new();
  let start_node = Node {
    position: start,
    orientation: start_orientation,
    cost: 0,
    priority: heuristic((start.x, start.y), (goal.x, goal.y)),
  };
  open_set.push(start_node);

  let mut came_from: HashMap<PointDirectionPair, Vec<PointDirectionPair>> =
    HashMap::new();
  let mut cost_so_far: HashMap<PointDirectionPair, u32> = HashMap::new();

  came_from
    .insert((start, start_orientation), vec![(start, start_orientation)]);
  cost_so_far.insert((start, start_orientation), 0);

  while let Some(current_node) = open_set.pop() {
    let current = current_node.position;
    let current_orientation = current_node.orientation;

    if current == goal {
      let total_cost = current_node.cost;
      let goal_state = (current, current_orientation);
      let start_state = (start, start_orientation);

      let mut all_paths = Vec::new();
      let mut path = Vec::new();

      fn backtrack(
        came_from: &HashMap<PointDirectionPair, Vec<PointDirectionPair>>,
        current: PointDirectionPair,
        start: PointDirectionPair,
        path: &mut Vec<Point>,
        all_paths: &mut Vec<Vec<Point>>,
      ) {
        if current == start {
          let mut complete_path = path.clone();
          complete_path.push(current.0);
          complete_path.reverse();
          all_paths.push(complete_path);
          return;
        }

        if let Some(predecessors) = came_from.get(&current) {
          for predecessor in predecessors {
            path.push(predecessor.0);
            backtrack(came_from, *predecessor, start, path, all_paths);
            path.pop();
          }
        }
      }

      backtrack(
        &came_from,
        goal_state,
        start_state,
        &mut path,
        &mut all_paths,
      );

      return Some((all_paths, total_cost));
    }

    let successors = get_successors(&current, &current_orientation, maze);
    for (successor_position, successor_orientation, action_cost) in successors {
      let new_cost = cost_so_far[&(current, current_orientation)] + action_cost;
      let successor_state = (successor_position, successor_orientation);

      if !cost_so_far.contains_key(&successor_state)
        || new_cost < cost_so_far[&successor_state]
      {
        cost_so_far.insert(successor_state, new_cost);
        let priority = new_cost
          + heuristic(
            (successor_position.x, successor_position.y),
            (goal.x, goal.y),
          );
        open_set.push(Node {
          position: successor_position,
          orientation: successor_orientation,
          cost: new_cost,
          priority,
        });
        came_from
          .entry(successor_state)
          .or_default()
          .push((current, current_orientation));
      } else if new_cost == cost_so_far[&successor_state] {
        // If the new_cost is equal, add the new predecessor
        came_from
          .entry(successor_state)
          .or_default()
          .push((current, current_orientation));
      }
    }
  }

  None // No path found
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
  #[cfg(feature = "part2")]
  use std::collections::HashSet;

  #[cfg(not(feature = "part2"))]
  use crate::a_star_search;
  #[cfg(feature = "part2")]
  use crate::a_star_search_multi;
  use crate::{
    src_provider, Cell, Consequent, Point, ProblemDefinition, DIRECTIONS,
  };

  pub fn extract() -> Result<ProblemDefinition, String> {
    src_provider()?
      .trim()
      .parse()
      .map_err(|_| "Error parsing grid".into())
  }

  pub fn transform(mut data: ProblemDefinition) -> Result<Consequent, String> {
    let (start, _) = data
      .iter::<Point>()
      .find(|(_, cell)| matches!(cell, Cell::Start))
      .ok_or_else(|| "No start point found".to_string())?;
    data.set_cell(start, Cell::Space);

    let (end, _) = data
      .iter::<Point>()
      .find(|(_, cell)| matches!(cell, Cell::End))
      .ok_or_else(|| "No end point found".to_string())?;

    let orientation = DIRECTIONS[1]; // East

    #[cfg(not(feature = "part2"))]
    {
      if let Some((path, total_cost)) =
        a_star_search(start, end, orientation, &data)
      {
        Ok(Consequent { path, total_cost })
      } else {
        Err("No path found".to_string())
      }
    }

    #[cfg(feature = "part2")]
    {
      if let Some((paths, total_cost)) =
        a_star_search_multi(start, end, orientation, &data)
      {
        let mut locations =
          paths.into_iter().flatten().collect::<HashSet<Point>>();
        locations.insert(end);
        Ok(Consequent { locations, total_cost })
      } else {
        Err("No path found".to_string())
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
