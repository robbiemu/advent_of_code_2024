#[cfg(feature = "part2")]
use std::collections::HashMap;
use std::fmt::Debug;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[cfg(not(feature = "part2"))]
const STEPS: usize = 25;
#[cfg(feature = "part2")]
const STEPS: usize = 75;

#[derive(PartialEq)]
pub struct PlutonicAutomata(String);

impl Debug for PlutonicAutomata {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "PA{}", self.0)
  }
}

impl PlutonicAutomata {
  fn get_state(&self) -> PlutonicState {
    match self.0.as_str() {
      "0" => PlutonicState::Zero,
      n if n.len() % 2 == 0 => PlutonicState::EvenDigits,
      _ => PlutonicState::TwentyTwentyFour,
    }
  }
  pub fn step(&self) -> Result<[Option<PlutonicAutomata>; 2], String> {
    match self.get_state() {
      PlutonicState::Zero => {
        Ok([Some(PlutonicAutomata("1".to_string())), None])
      }
      PlutonicState::EvenDigits => {
        let size = self.0.len() / 2;
        let left = self.0[..size].to_string();
        let right = {
          let trimmed = &self.0[size..].trim_start_matches('0');
          if trimmed.is_empty() {
            "0".to_string()
          } else {
            trimmed.to_string()
          }
        };

        Ok([Some(PlutonicAutomata(left)), Some(PlutonicAutomata(right))])
      }
      PlutonicState::TwentyTwentyFour => {
        let Ok(value) = self.0.parse::<usize>() else {
          return Err(format!("Failed to parse input {}", self.0));
        };
        let next = (value * 2024).to_string();
        Ok([Some(PlutonicAutomata(next)), None])
      }
    }
  }
}

#[derive(Debug, Default)]
enum PlutonicState {
  Zero,
  EvenDigits,
  #[default]
  TwentyTwentyFour,
}

pub type ProblemDefinition = Vec<PlutonicAutomata>;
#[cfg(not(feature = "part2"))]
pub type Consequent = Vec<PlutonicAutomata>;
#[cfg(feature = "part2")]
pub type Consequent = usize;


#[cfg(test)]
#[mry::mry]
pub fn get_steps() -> usize {
  STEPS
}
#[cfg(not(test))]
pub fn get_steps() -> usize {
  STEPS
}

#[cfg(feature = "part2")]
fn count_descendants(
  value: usize,
  steps: usize,
  cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
  if steps == 0 {
    return 1;
  }

  if let Some(&count) = cache.get(&(value, steps)) {
    return count;
  }

  let mut count = 0;

  match value {
    0 => {
      // Replace with 1
      count += count_descendants(1, steps - 1, cache);
    }
    _ => {
      let digits = value.to_string().len();
      if digits % 2 == 0 {
        // Split into two stones
        let size = digits / 2;
        let left = value.to_string()[..size].parse::<usize>().unwrap_or(0);
        let right = value.to_string()[size..].parse::<usize>().unwrap_or(0);
        count += count_descendants(left, steps - 1, cache);
        count += count_descendants(right, steps - 1, cache);
      } else {
        // Multiply by 2024
        let next = value * 2024;
        count += count_descendants(next, steps - 1, cache);
      }
    }
  }

  cache.insert((value, steps), count);
  count
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
  use crate::count_descendants;
  use crate::{
    get_steps, src_provider, Consequent, PlutonicAutomata, ProblemDefinition,
  };
  #[cfg(feature = "part2")]
  use std::collections::HashMap;

  pub fn extract() -> Result<ProblemDefinition, String> {
    Ok(
      src_provider()?
        .split_whitespace()
        .map(|s| PlutonicAutomata(s.to_string()))
        .collect(),
    )
  }

  #[cfg(not(feature = "part2"))]
  pub fn transform(mut data: ProblemDefinition) -> Result<Consequent, String> {
    let steps = get_steps();
    for _ in 0..steps {
      data = data
        .into_iter()
        .filter_map(|p| p.step().ok())
        .flat_map(|[left, right]| [left, right].into_iter().flatten())
        .collect();
    }

    Ok(data)
  }
  #[cfg(feature = "part2")]
  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let mut cache = HashMap::new();
    let mut total = 0;
    let steps = get_steps();
    for stone in data {
      let value = stone.0.parse::<usize>().unwrap_or(0);
      total += count_descendants(value, steps, &mut cache);
    }

    Ok(total)
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(consequent) => {
        #[cfg(not(feature = "part2"))]
        println!("Result: {:?}", consequent.len());
        #[cfg(feature = "part2")]
        println!("Result: {consequent}");

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
  #[cfg(all(feature = "sample", not(feature = "part2")))]
  #[mry::lock(get_steps)]
  #[test]
  fn test_transform() {
    mock_get_steps().returns(6);

    let data = extract().expect("Failed to extract data");
    let result = transform(data);

    assert_eq!(
      result,
      Ok(
        [
          2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2,
          8, 6, 7, 6, 0, 3, 2
        ]
        .into_iter()
        .map(|x| PlutonicAutomata(x.to_string()))
        .collect::<Vec<PlutonicAutomata>>()
      )
    );
  }

  #[cfg(all(feature = "sample", not(feature = "part2")))]
  #[mry::lock(get_steps)]
  #[test]
  fn test_transform_to_step_25() {
    mock_get_steps().returns(25);

    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("Failed to transform data");

    assert_eq!(result.len(), 55312);
  }
  #[cfg(all(feature = "sample", feature = "part2"))]
  #[mry::lock(get_steps)]
  #[test]
  fn test_transform_to_step_25_part_2() {
    mock_get_steps().returns(25);

    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("Failed to transform data");

    assert_eq!(result, 55312);
  }

  // MARK load
}
