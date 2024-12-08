use game_grid::GridPosition;
use std::ops::{Add, Sub};


#[derive(GridPosition, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

// Implement Add for Point
impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Point { x: self.x + other.x, y: self.y + other.y }
  }
}

// Implement Sub for Point
impl Sub for Point {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Point { x: self.x - other.x, y: self.y - other.y }
  }
}

impl game_grid::GridPosition for &Point {
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
