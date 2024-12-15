#![allow(dead_code)]

pub mod prelude {
  pub const X_DIM: usize = 101;
  pub const Y_DIM: usize = 103;

  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct Point<T> {
    pub x: T,
    pub y: T,
  }

  #[derive(Debug, PartialEq, Eq)]
  pub struct Robot {
    pub position: Point<u8>,
    pub velocity: Point<i8>,
  }
}

use prelude::*;


const SIMULATION_DURATION: usize = 100;


pub struct ProblemDefinition {
  pub robots: Vec<Robot>,
  pub duration: usize,
}

pub fn get_duration() -> usize {
  SIMULATION_DURATION
}
