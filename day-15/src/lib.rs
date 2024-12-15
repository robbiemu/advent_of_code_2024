use game_grid::{Grid, GridCell, GridPosition, ParseCellError};
use std::convert::TryFrom;

#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");


#[derive(GridCell, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Cell {
  #[cell('.')]
  #[default]
  Empty,
  #[cell('#')]
  Wall,
  #[cfg(not(feature = "part2"))]
  #[cell('O')]
  Box,
  #[cfg(feature = "part2")]
  #[cell('[')]
  BoxLeft,
  #[cfg(feature = "part2")]
  #[cell(']')]
  BoxRight,
  #[cell('@')]
  Robot,
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

impl Point {
  #[cfg(feature = "part2")]
  fn process_move(&mut self, direction: &mut Point, map: &mut Grid<Cell>) {
    let next_pos = Point { x: direction.x + self.x, y: direction.y + self.y };

    let Some(box_positions) = self.bfs(&next_pos, direction, map) else {
      return;
    };
    box_positions.iter().rev().for_each(|&box_pos| {
      let cell = map[box_pos];
      map.set_cell(box_pos, Cell::Empty);
      map.set_cell(next_pos, cell);
    });
    map.set_cell(self.to_owned(), Cell::Empty);
    map.set_cell(next_pos, Cell::Robot);
    self.x = next_pos.x;
    self.y = next_pos.y;
  }

  #[cfg(feature = "part2")]
  fn bfs(
    &self,
    next_pos: &Point,
    direction: &Point,
    map: &Grid<Cell>,
  ) -> Option<Vec<Point>> {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    let mut visited = std::collections::HashSet::new();
    let mut box_positions = Vec::new();

    queue.push_back(*next_pos);
    visited.insert(*next_pos);

    while let Some(current_pos) = queue.pop_front() {
      let current_cell = map[current_pos];

      match current_cell {
        Cell::Empty => {
          // Continue exploring in the same direction
          let next_pos = Point {
            x: current_pos.x + direction.x,
            y: current_pos.y + direction.y,
          };

          if map.is_in_bounds(next_pos) && !visited.contains(&next_pos) {
            queue.push_back(next_pos);
            visited.insert(next_pos);
          }
        }
        Cell::BoxLeft | Cell::BoxRight => {
          // Find the pair of the box
          let pair_cell = match current_cell {
            Cell::BoxLeft => Cell::BoxRight,
            Cell::BoxRight => Cell::BoxLeft,
            _ => unreachable!(),
          };

          let pair_pos = Point {
            x: current_pos.x + direction.x,
            y: current_pos.y + direction.y,
          };

          if map.is_in_bounds(pair_pos) && map[pair_pos] == pair_cell {
            // Ensure the pair is in the queue
            if !visited.contains(&pair_pos) {
              queue.push_back(pair_pos);
              visited.insert(pair_pos);
            }

            // Queue the next position after the pair
            let next_after_pair = Point {
              x: pair_pos.x + direction.x,
              y: pair_pos.y + direction.y,
            };

            if map.is_in_bounds(next_after_pair)
              && !visited.contains(&next_after_pair)
            {
              queue.push_back(next_after_pair);
              visited.insert(next_after_pair);
            }

            // Collect the box positions
            box_positions.push(current_pos);
            box_positions.push(pair_pos);
          } else {
            // If no valid pair is found, return None
            return None;
          }
        }
        Cell::Wall => {
          // If a wall is encountered, return None
          return None;
        }
        _ => {
          // For any other cell type, return None
          return None;
        }
      }
    }

    Some(box_positions)
  }

  #[cfg(not(feature = "part2"))]
  fn process_move(&mut self, direction: &mut Point, map: &mut Grid<Cell>) {
    let next_pos = Point { x: direction.x + self.x, y: direction.y + self.y };

    // Check if the next position is a box
    let mut box_positions = Vec::new();
    if map.is_in_bounds(next_pos) {
      if map[next_pos] == Cell::Box {
        // Find the end of the chain of boxes
        box_positions.push(next_pos);
        let mut last_box_pos = next_pos;
        loop {
          let next_box_pos = Point {
            x: last_box_pos.x + direction.x,
            y: last_box_pos.y + direction.y,
          };
          match map[next_box_pos] {
            Cell::Box => {
              box_positions.push(next_box_pos);
              last_box_pos = next_box_pos;
            }
            Cell::Empty => {
              // Found an empty space, break the loop
              break;
            }
            _ => {
              // Wall, prevent movement
              return;
            }
          }
        }

        // Move all boxes in the chain
        for pos in box_positions.iter().rev() {
          let new_pos =
            Point { x: pos.x + direction.x, y: pos.y + direction.y };
          map.set_cell(*pos, Cell::Empty);
          map.set_cell(new_pos, Cell::Box);
        }

        // Move the robot to the first box's original position
        map.set_cell(self.to_owned(), Cell::Empty);
        map.set_cell(next_pos, Cell::Robot);
        self.x = next_pos.x;
        self.y = next_pos.y;
      } else if map[next_pos] == Cell::Empty {
        // Move the robot to the empty cell
        map.set_cell(self.to_owned(), Cell::Empty);
        map.set_cell(next_pos, Cell::Robot);
        self.x = next_pos.x;
        self.y = next_pos.y;
      }
    }
  }
}

impl TryFrom<char> for Point {
  type Error = String;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c {
      '^' => Ok(Point { x: 0, y: -1 }),
      'v' => Ok(Point { x: 0, y: 1 }),
      '<' => Ok(Point { x: -1, y: 0 }),
      '>' => Ok(Point { x: 1, y: 0 }),
      _ => Err(format!("Invalid character for Point conversion: {}", c)),
    }
  }
}

pub struct ProblemDefinition {
  pub map: Grid<Cell>,
  pub movements: Vec<Point>,
}
#[cfg(not(feature = "part2"))]
pub type Consequent = Vec<Point>;
#[cfg(feature = "part2")]
pub type Consequent = Vec<(Point, Point)>;


#[cfg(feature = "part2")]
fn widen_map(input: String) -> Result<String, String> {
  let Some((map_str, movement_str)) = input.split_once("\n\n") else {
    return Err("Invalid input format, could not split".to_string());
  };

  let widened_map = map_str
    .chars()
    .map(|c| match c {
      '#' => Ok("##"),
      'O' => Ok("[]"),
      '.' => Ok(".."),
      '@' => Ok("@."),
      '\n' => Ok("\n"), // Preserve line breaks
      _ => Err(format!("Parse error on character '{c}'")),
    })
    .collect::<Result<Vec<&str>, String>>()?
    .concat(); // Concatenate the transformed map

  Ok(format!("{}\n\n{}", widened_map, movement_str.trim()))
}


fn gps_coordinate(point: &Point) -> i32 {
  point.y * 100 + point.x
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
  use crate::widen_map;
  use crate::{
    gps_coordinate, src_provider, Cell, Consequent, Point, ProblemDefinition,
  };

  pub fn extract() -> Result<ProblemDefinition, String> {
    #[cfg(not(feature = "part2"))]
    let input = src_provider()?;
    #[cfg(feature = "part2")]
    let input = widen_map(src_provider()?)?;
    let Some((map_str, movement_str)) = input.split_once("\n\n") else {
      return Err("Invalid input format, could not split".to_string());
    };

    Ok(ProblemDefinition {
      map: map_str
        .trim()
        .parse()
        .map_err(|_| "Error parsing grid".to_string())?,
      movements: movement_str
        .trim()
        .chars()
        .filter_map(|c| {
          if c.is_whitespace() {
            None
          } else {
            Some(
              Point::try_from(c)
                .map_err(|_| format!("failed to parse movement '{c}'")),
            )
          }
        })
        .collect::<Result<Vec<_>, _>>()?,
    })
  }

  pub fn transform(mut data: ProblemDefinition) -> Result<Consequent, String> {
    let Some((mut location, _)) =
      data.map.iter::<Point>().find(|(_, c)| *c == Cell::Robot)
    else {
      return Err("Robot not found in the map".to_string());
    };

    data.movements.iter().for_each(|motion| {
      location.process_move(&mut motion.clone(), &mut data.map);
    });

    Ok(
      #[cfg(not(feature = "part2"))]
      data
        .map
        .iter::<Point>()
        .filter_map(|(p, c)| if c == Cell::Box { Some(p) } else { None })
        .collect::<Vec<_>>(),
      #[cfg(feature = "part2")]
      data
        .map
        .iter::<Point>()
        .filter_map(|(p, c)| match c {
          Cell::BoxLeft => {
            let right_pos = Point { x: p.x + 1, y: p.y };
            if data.map[right_pos] == Cell::BoxRight {
              Some((p, right_pos))
            } else {
              None
            }
          }
          Cell::BoxRight => None, // Avoid duplicates; handle pairs via BoxLeft
          _ => None,
        })
        .collect::<Vec<_>>(),
    )
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(consequent) => {
        #[cfg(not(feature = "part2"))]
        let total = consequent.iter().map(gps_coordinate).sum::<i32>();
        #[cfg(feature = "part2")]
        let total = consequent
          .iter()
          .map(|(left, right)| gps_coordinate(left).min(gps_coordinate(right)))
          .sum::<i32>();

        println!("Consequent: {:?}", total);

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
  #[cfg(not(feature = "part2"))]
  #[test]
  #[mry::lock(src_provider)] // Lock the function for mocking.
  fn test_extract() {
    let input_str = "#.
O@

<^v>v^<>";
    mock_src_provider().returns(Ok(input_str.to_string()));

    let result = extract();
    match result {
      Ok(data) => {
        assert_eq!(
          data.map.iter().collect::<Vec<_>>(),
          vec![
            (Point { x: 0, y: 0 }, Cell::Wall),
            (Point { x: 1, y: 0 }, Cell::Empty),
            (Point { x: 0, y: 1 }, Cell::Box),
            (Point { x: 1, y: 1 }, Cell::Robot)
          ]
        );
        assert_eq!(
          data.movements,
          vec![
            Point { x: -1, y: 0 },
            Point { x: 0, y: -1 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: -1 },
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 }
          ]
        );
      }
      Err(e) => {
        panic!("Failed to extract: {}", e);
      }
    }
  }

  // MARK transform
  #[cfg(not(feature = "part2"))]
  #[test]
  #[mry::lock(src_provider)]
  fn test_transform_mini() {
    mock_src_provider()
      .returns(Ok(include_str!("../sample.mini.txt").to_string()));

    let data = extract().expect("Failed to extract");
    let result = transform(data).expect("Failed to transform");

    dbg!(&result);

    let boxes = [
      Point { x: 5, y: 1 },
      Point { x: 6, y: 1 },
      Point { x: 6, y: 3 },
      Point { x: 3, y: 4 },
      Point { x: 4, y: 5 },
      Point { x: 4, y: 6 },
    ];
    assert!(result.iter().all(|b| boxes.contains(b)));
    assert_eq!(result.iter().map(gps_coordinate).sum::<i32>(), 2028);
  }

  #[cfg(all(feature = "sample", not(feature = "part2")))]
  #[test]
  fn test_transform() {
    let data = extract().expect("Failed to extract");
    let result = transform(data).expect("Failed to transform");

    dbg!(&result);

    assert_eq!(result.iter().map(gps_coordinate).sum::<i32>(), 10092);
  }

  #[cfg(all(feature = "sample", feature = "part2"))]
  #[test]
  fn test_transform() {
    let data = extract().expect("Failed to extract");
    let result = transform(data).expect("Failed to transform");

    dbg!(&result);

    assert_eq!(
      result
        .iter()
        .map(|(left, right)| gps_coordinate(left).min(gps_coordinate(right)))
        .sum::<i32>(),
      9021
    );
  }

  // MARK load
}
