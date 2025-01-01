// PRE Z3 SOLVER MODEL OF THE SOLUTION

const DATA: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
  x: i64,
  y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Key {
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Zero,
  A,
  Left,
  Right,
  Up,
  Down,
}

impl std::fmt::Display for Key {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Key::One => write!(f, "1"),
      Key::Two => write!(f, "2"),
      Key::Three => write!(f, "3"),
      Key::Four => write!(f, "4"),
      Key::Five => write!(f, "5"),
      Key::Six => write!(f, "6"),
      Key::Seven => write!(f, "7"),
      Key::Eight => write!(f, "8"),
      Key::Nine => write!(f, "9"),
      Key::Zero => write!(f, "0"),
      Key::A => write!(f, "A"),
      Key::Left => write!(f, "<"),
      Key::Right => write!(f, ">"),
      Key::Up => write!(f, "^"),
      Key::Down => write!(f, "v"),
    }
  }
}

impl Key {
  fn from_u64(value: u64) -> Option<Key> {
    match value {
      0 => Some(Key::A),
      1 => Some(Key::Left),
      2 => Some(Key::Right),
      3 => Some(Key::Up),
      4 => Some(Key::Down),
      _ => None,
    }
  }
}

trait Bounded {
  fn is_in_bounds(&self, position: Position) -> bool;
}

struct NumericKeypad;

impl Bounded for NumericKeypad {
  fn is_in_bounds(&self, position: Position) -> bool {
    match position {
      Position { x, y } if x > 0 && x < 3 && y == 3 => true,
      Position { x, y } if x >= 0 && x < 3 && y >= 0 && y < 3 => true,
      _ => false,
    }
  }
}

impl NumericKeypad {
  fn get_button(&self, position: Position) -> Option<Key> {
    match position {
      Position { x, y } if x == 0 && y == 0 => Some(Key::Seven),
      Position { x, y } if x == 1 && y == 0 => Some(Key::Eight),
      Position { x, y } if x == 2 && y == 0 => Some(Key::Nine),
      Position { x, y } if x == 0 && y == 1 => Some(Key::Four),
      Position { x, y } if x == 1 && y == 1 => Some(Key::Five),
      Position { x, y } if x == 2 && y == 1 => Some(Key::Six),
      Position { x, y } if x == 0 && y == 2 => Some(Key::One),
      Position { x, y } if x == 1 && y == 2 => Some(Key::Two),
      Position { x, y } if x == 2 && y == 2 => Some(Key::Three),
      Position { x, y } if x == 1 && y == 3 => Some(Key::Zero),
      Position { x, y } if x == 2 && y == 3 => Some(Key::A),
      _ => None,
    }
  }
}

struct DirectionKeypad;

impl Bounded for DirectionKeypad {
  fn is_in_bounds(&self, position: Position) -> bool {
    match position {
      Position { x, y } if x >= 0 && x < 3 && y == 1 => true,
      Position { x, y } if x > 0 && x < 3 && y == 0 => true,
      _ => false,
    }
  }
}

impl DirectionKeypad {
  fn get_button(&self, position: Position) -> Option<Key> {
    match position {
      Position { x, y } if x == 1 && y == 0 => Some(Key::Up),
      Position { x, y } if x == 2 && y == 0 => Some(Key::A),
      Position { x, y } if x == 0 && y == 1 => Some(Key::Left),
      Position { x, y } if x == 1 && y == 1 => Some(Key::Down),
      Position { x, y } if x == 2 && y == 1 => Some(Key::Right),
      _ => None,
    }
  }
}

enum Keypad {
  Numeric(NumericKeypad),
  Direction(DirectionKeypad),
}

struct Robot {
  position: Position,
  keypad: Keypad,
}

impl Robot {
  fn press(&self) -> Option<Key> {
    match &self.keypad {
      Keypad::Numeric(keypad) => keypad.get_button(self.position),
      Keypad::Direction(keypad) => keypad.get_button(self.position),
    }
  }

  fn change_position(&mut self, direction: Position) -> Option<()> {
    let new_position = Position {
      x: self.position.x + direction.x,
      y: self.position.y + direction.y,
    };
    if match &self.keypad {
      Keypad::Numeric(keypad) => keypad.is_in_bounds(new_position),
      Keypad::Direction(keypad) => keypad.is_in_bounds(new_position),
    } {
      self.position = new_position;
      Some(())
    } else {
      None
    }
  }
}

struct RobotStack {
  robots: Vec<Robot>,
}

impl RobotStack {
  fn trigger(&mut self, key: Key) -> Result<Option<Key>, String> {
    let mut current_key = key;

    for i in 0..self.robots.len() {
      match current_key {
        Key::Up => {
          self.robots[i]
            .change_position(Position { x: 0, y: -1 })
            .ok_or_else(|| format!("Move out of bounds for robot {i}"))?;
          return Ok(None);
        }
        Key::Down => {
          self.robots[i]
            .change_position(Position { x: 0, y: 1 })
            .ok_or_else(|| format!("Move out of bounds for robot {i}"))?;
          return Ok(None);
        }
        Key::Left => {
          self.robots[i]
            .change_position(Position { x: -1, y: 0 })
            .ok_or_else(|| format!("Move out of bounds for robot {i}"))?;
          return Ok(None);
        }
        Key::Right => {
          self.robots[i]
            .change_position(Position { x: 1, y: 0 })
            .ok_or_else(|| format!("Move out of bounds for robot {i}"))?;
          return Ok(None);
        }
        Key::A => {
          let Some(next_key) = self.robots[i].press() else {
            return Err(format!(
              "Invalid key activation result for {current_key} at robot {i}"
            ));
          };
          current_key = next_key;
        }
        _ => return Err(format!("Invalid key {current_key} for robot {i}")),
      }
    }

    Ok(Some(current_key))
  }
}

type Value = u64;

pub struct ProblemDefinition {}
pub type Consequent = String;


fn evaluate_choice(
  choice: u64,
  input_index: usize,
  target_sequence: &[Key],
  stack: &mut RobotStack,
) -> Option<Value> {
  // a return of None means the choice is invalid
  // a return of 0 means the evaluation of the input_index is not yet complete
  // a return of 1 means the evaluation of the input_index is successful

  let key = Key::from_u64(choice)?;
  match stack.trigger(key) {
    Ok(Some(next_key)) => {
      if next_key == target_sequence[input_index] {
        Some(1)
      } else {
        None
      }
    }
    Ok(None) => Some(0),
    Err(e) => {
      eprintln!("{}", e);

      None
    }
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
  use crate::{src_provider, Consequent, ProblemDefinition};

  pub fn extract() -> Result<ProblemDefinition, String> {
    todo!()
  }

  pub fn transform(_data: ProblemDefinition) -> Result<Consequent, String> {
    todo!()
  }

  pub fn load(_result: Result<Consequent, String>) -> Result<(), String> {
    todo!()
  }
}


#[cfg(test)]
mod tests {
  //  #[allow(unused_imports)]
  //  use super::{prelude::*, *};

  // MARK extract
  // #[mry::lock(src_provider)] // Lock the function for mocking.

  // MARK transform

  // MARK load
}
