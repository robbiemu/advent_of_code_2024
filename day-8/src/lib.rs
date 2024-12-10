#[allow(unused_imports)]
use game_grid::{Grid, GridCell, GridPosition, ParseCellError};
use std::collections::HashSet;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

pub type ProblemDefinition = Grid<Cell>;
pub type Consequent = HashSet<Point>;

#[derive(GridCell, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Cell {
  #[cell('.')]
  #[default]
  Empty,
  #[cell('a'..='z' | 'A'..='Z' | '0'..='9')]
  Antenna(char),
}

#[derive(GridPosition, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl GridPosition for &Point {
  fn new(x: i32, y: i32) -> Self {
    Box::leak(Box::new(Point { x, y }))
  }

  fn x(&self) -> i32 {
    self.x
  }

  fn y(&self) -> i32 {
    self.y
  }
}

#[cfg(not(feature = "part2"))]
fn antinodes(a: Point, b: Point) -> [Point; 2] {
  let antinode1 = Point { x: 2 * a.x - b.x, y: 2 * a.y - b.y };
  let antinode2 = Point { x: 2 * b.x - a.x, y: 2 * b.y - a.y };

  [antinode1, antinode2]
}
#[cfg(feature = "part2")]
fn antinodes(a: Point, b: Point, data: &ProblemDefinition) -> HashSet<Point> {
  use num::integer::gcd;
  let mut points = HashSet::new();

  let displacement = Point { x: b.x - a.x, y: b.y - a.y };

  // Compute the step size using GCD
  let gcd_value = gcd(displacement.x.abs(), displacement.y.abs());
  if gcd_value == 0 {
    points.insert(a);
    return points;
  }
  let step =
    Point { x: displacement.x / gcd_value, y: displacement.y / gcd_value };

  let mut forward = a;
  let mut backward = a;
  loop {
    let mut did_insert = false;

    forward = Point { x: forward.x + step.x, y: forward.y + step.y };
    if data.is_in_bounds(forward) {
      points.insert(forward);
      did_insert = true;
    }

    backward = Point { x: backward.x - step.x, y: backward.y - step.y };
    if data.is_in_bounds(backward) {
      points.insert(backward);
      did_insert = true;
    }

    if !did_insert {
      break;
    }
  }

  // Include the original points
  points.insert(a);
  points.insert(b);

  points
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
  use itertools::Itertools;
  use std::collections::HashMap;

  use crate::{
    antinodes, src_provider, Cell, Consequent, Point, ProblemDefinition,
  };

  pub fn extract() -> Result<ProblemDefinition, String> {
    src_provider()?
      .parse()
      .map_err(|_| "Error parsing grid".into())
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let mapped = data.iter::<Point>().fold(
      HashMap::<char, Vec<Point>>::new(),
      |mut acc, (point, cell)| {
        if let Cell::Antenna(left_char) = cell {
          acc.entry(left_char).or_default().push(point);
        }
        acc
      },
    );

    Ok(
      mapped
        .values()
        .flat_map(|points| {
          points
            .iter()
            .combinations(2)
            .flat_map(|pair| {
              let [left, right] = &pair[..] else {
                unreachable!()
              };

              #[cfg(not(feature = "part2"))]
              return antinodes(**left, **right)
                .iter()
                .filter(|p| data.is_in_bounds(*p))
                .cloned()
                .collect::<Vec<Point>>();
              #[cfg(feature = "part2")]
              return antinodes(**left, **right, &data)
                .iter()
                .cloned()
                .collect::<Vec<Point>>();
            })
            .collect::<Vec<Point>>()
        })
        .collect(),
    )
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(consequent) => println!("Consequent: {:?}", consequent.len()),
      Err(e) => println!("Error: {}", e),
    }

    Ok(())
  }
}


#[cfg(test)]
mod tests {
  use super::{prelude::*, *};

  fn render_grid(data: &ProblemDefinition, results: &Consequent) -> String {
    let mut rendered = String::new();

    let mut current_y = 0; // Track the current row (y-coordinate)

    for (point, cell) in data.iter::<Point>() {
      // Add a newline if we move to a new row
      if point.y != current_y {
        rendered.push('\n');
        current_y = point.y;
      }

      // Determine what to render at this point
      if results.contains(&point) {
        match cell {
          Cell::Antenna(c) => rendered.push(c), // Keep antenna character
          Cell::Empty => rendered.push('#'),    // Render antinodes as #
        }
      } else {
        match cell {
          Cell::Antenna(c) => rendered.push(c),
          Cell::Empty => rendered.push('.'),
        }
      }
    }

    rendered.push('\n');

    rendered
  }


  // MARK extract
  #[test]
  #[mry::lock(src_provider)] // Lock the function for mocking.
  fn test_etract() {
    let data = "A.
0a";

    mock_src_provider().returns(Ok(data.to_string()));
    let result = extract().expect("Failed to extract data");

    assert_eq!(
      result.iter().collect::<Vec<(Point, Cell)>>(),
      vec![
        (Point::new(0, 0), Cell::Antenna('A')),
        (Point::new(1, 0), Cell::Empty),
        (Point::new(0, 1), Cell::Antenna('0')),
        (Point::new(1, 1), Cell::Antenna('a')),
      ]
    );
  }

  // MARK transform
  #[cfg(all(feature = "sample", not(feature = "part2")))]
  #[test]
  fn test_transform() {
    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("Failed to transform data");

    assert_eq!(result.len(), 14);
  }

  #[cfg(all(feature = "sample", feature = "part2"))]
  #[test]
  fn test_transform() {
    let data = extract().expect("Failed to extract data");
    let result = transform(data.clone()).expect("Failed to transform data");

    println!("{}", render_grid(&data, &result));

    assert_eq!(result.len(), 34);
  }

  // MARK load
}
