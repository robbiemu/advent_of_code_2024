use game_grid::{Grid, GridCell, GridPosition, ParseCellError};
#[cfg(feature = "part2")]
use linked_hash_set::LinkedHashSet;
#[cfg(feature = "part2")]
use std::collections::{HashSet, VecDeque};
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
    let Some(box_positions) = self.bfs(direction, map) else {
      return;
    };

    #[cfg(feature = "debug")]
    dbg!(&box_positions);

    box_positions.iter().rev().for_each(|&box_pos| {
      let cell = map[box_pos];
      let cell_next_pos =
        Point { x: box_pos.x + direction.x, y: box_pos.y + direction.y };
      map.set_cell(cell_next_pos, cell);
      map.set_cell(box_pos, Cell::Empty);
    });

    let next_pos = Point { x: direction.x + self.x, y: direction.y + self.y };

    map.set_cell(self.to_owned(), Cell::Empty);
    map.set_cell(next_pos, Cell::Robot);
    self.x = next_pos.x;
    self.y = next_pos.y;
  }

  #[cfg(feature = "part2")]
  fn bfs(
    &self,
    direction: &Point,
    map: &Grid<Cell>,
  ) -> Option<LinkedHashSet<Point>> {
    let next_pos = Point { x: direction.x + self.x, y: direction.y + self.y };

    let mut box_positions = LinkedHashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(next_pos);
    visited.insert(next_pos);
    while let Some(current_pos) = queue.pop_front() {
      if !map.is_in_bounds(current_pos) {
        return None;
      }
      let current_cell = map[current_pos];

      match current_cell {
        Cell::Empty => {
          // this progressional check is satisfied
          continue;
        }
        Cell::BoxLeft | Cell::BoxRight => {
          // Identify the paired box cell horizontally
          let pair_pos = if current_cell == Cell::BoxLeft {
            Point { x: current_pos.x + 1, y: current_pos.y }
          } else {
            Point { x: current_pos.x - 1, y: current_pos.y }
          };

          // no need to verify that the paired cell exists and matches
          let next_of_current = Point {
            x: current_pos.x + direction.x,
            y: current_pos.y + direction.y,
          };
          let next_of_pair =
            Point { x: pair_pos.x + direction.x, y: pair_pos.y + direction.y };

          if next_of_current != pair_pos && visited.insert(next_of_current) {
            queue.push_back(next_of_current);
          }
          if visited.insert(next_of_pair) {
            queue.push_back(next_of_pair);
          }
          // Collect the current and paired box positions
          box_positions.insert(current_pos);
          box_positions.insert(pair_pos);
        }
        Cell::Wall => {
          // Encountering a wall blocks the move
          return None;
        }
        _ => {
          unreachable!();
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

#[cfg(feature = "debug")]
struct WrappedGrid(Grid<Cell>);
#[cfg(feature = "debug")]
impl std::fmt::Display for WrappedGrid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in 0..self.0.height() {
      for x in 0..self.0.width() {
        let cell = self.0[Point { x: x as i32, y: y as i32 }];
        let symbol = match cell {
          Cell::Empty => '.',
          Cell::Wall => '#',
          #[cfg(feature = "part2")]
          Cell::BoxLeft => '[',
          #[cfg(feature = "part2")]
          Cell::BoxRight => ']',
          #[cfg(not(feature = "part2"))]
          Cell::Box => 'O',
          Cell::Robot => '@',
        };
        write!(f, "{}", symbol)?;
      }
      writeln!(f)?; // Add a newline after each row
    }
    Ok(())
  }
}

pub struct ProblemDefinition {
  pub map: Grid<Cell>,
  pub movements: Vec<Point>,
}
#[cfg(not(feature = "part2"))]
pub type Consequent = Vec<Point>;
#[cfg(feature = "part2")]
pub struct Consequent {
  boxes: Vec<(Point, Point)>,
  dims: Point,
}


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

#[cfg(not(feature = "part2"))]
fn gps_coordinate(point: &Point) -> i32 {
  point.y * 100 + point.x
}
#[cfg(feature = "part2")]
fn gps_coordinate(left: &Point, right: &Point, dims: &Point) -> i32 {
  // Determine the bounding box of the box pair
  let min_x = left.x.min(right.x);
  let max_x = left.x.max(right.x);
  let min_y = left.y.min(right.y);
  let max_y = left.y.max(right.y);

  // Calculate distances to all edges
  let top_distance = min_y;
  let bottom_distance = dims.y - max_y - 1; // Subtract 1 because the box is 1 unit tall
  let left_distance = min_x;
  let right_distance = dims.x - max_x; // Subtract 1 because the box is 1 unit wide

  let min_distance = top_distance
    .min(bottom_distance)
    .min(left_distance)
    .min(right_distance);

  if min_distance == top_distance {
    100 * top_distance + min_x
  } else if min_distance == bottom_distance {
    100 * (dims.y - bottom_distance - 1) + min_x
  } else if min_distance == left_distance {
    100 * min_y + left_distance
  } else {
    100 * min_y + (dims.x - right_distance - 1)
  }
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
  #[cfg(feature = "debug")]
  use crate::WrappedGrid;
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
      #[cfg(feature = "debug")]
      {
        let wg = WrappedGrid(data.map.clone());
        let dir = match motion {
          p if p.x == 1 => '>',
          p if p.x == -1 => '<',
          p if p.y == 1 => 'v',
          p if p.y == -1 => '^',
          _ => unreachable!(),
        };
        println!("{}\n\t{}", wg, dir);
      }
      location.process_move(&mut motion.clone(), &mut data.map);
    });

    #[cfg(feature = "debug")]
    {
      let wg = WrappedGrid(data.map.clone());
      println!("{}", wg);
    }

    Ok(
      #[cfg(not(feature = "part2"))]
      data
        .map
        .iter::<Point>()
        .filter_map(|(p, c)| if c == Cell::Box { Some(p) } else { None })
        .collect::<Vec<_>>(),
      #[cfg(feature = "part2")]
      {
        let boxes = data
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
          .collect::<Vec<_>>();

        Consequent {
          boxes,
          dims: Point {
            x: data.map.width() as i32,
            y: data.map.height() as i32,
          },
        }
      },
    )
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(consequent) => {
        #[cfg(not(feature = "part2"))]
        let total = consequent.iter().map(gps_coordinate).sum::<i32>();
        #[cfg(feature = "part2")]
        let total = consequent
          .boxes
          .iter()
          .map(|(left, right)| gps_coordinate(left, right, &consequent.dims))
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
  fn test_transform_part2() {
    let data = extract().expect("Failed to extract");
    let result = transform(data).expect("Failed to transform");

    assert_eq!(
      result
        .boxes
        .iter()
        .map(|(left, right)| gps_coordinate(left, right, &result.dims))
        .sum::<i32>(),
      9021
    );
  }

  // MARK load
}
