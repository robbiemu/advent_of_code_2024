use std::collections::HashSet;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
  Add,
  Multiply,
  #[cfg(feature = "part2")]
  Concatenation,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Equation {
  result: isize,
  values: Vec<isize>,
}

impl Equation {
  fn is_valid_under(&self, operations: &[Operator]) -> Result<bool, ()> {
    self
      .values
      .iter()
      .try_fold(HashSet::from([0isize]), |acc, &value| {
        let mut new_set = HashSet::new();
        for prev in acc {
          for op in operations {
            let new_value = match op {
              Operator::Add => prev.checked_add(value),
              Operator::Multiply => prev.checked_mul(value),
              #[cfg(feature = "part2")]
              Operator::Concatenation => {
                let mut joined = prev.to_string();
                joined.push_str(&value.to_string());
                joined.parse::<isize>().ok()
              }
            };

            // Only insert if the operation succeeded (no overflow) and doesn't exceed the result
            if let Some(valid_value) = new_value {
              if valid_value <= self.result {
                new_set.insert(valid_value);
              }
            }
          }
        }
        Ok(new_set)
      })
      .map(|final_set| final_set.contains(&self.result))
  }
}

pub type ProblemDefinition = Vec<Equation>;
pub type Consequent = Vec<isize>;


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
  use crate::{
    src_provider, Consequent, Equation, Operator, ProblemDefinition,
  };

  pub fn extract() -> Result<ProblemDefinition, String> {
    src_provider()?
      .lines()
      .enumerate()
      .map(|(line_number, line)| {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
          return Err(format!(
            "Invalid line format at line {}: '{}'",
            line_number + 1,
            line
          ));
        }

        let result = parts[0].parse::<isize>().map_err(|e| {
          format!(
            "Failed to parse result at line {} ('{}'): {:?}",
            line_number + 1,
            parts[0],
            e
          )
        })?;

        let values = parts[1]
          .split_whitespace()
          .enumerate()
          .map(|(value_index, value)| {
            value.parse::<isize>().map_err(|e| {
              format!(
                "Failed to parse value at line {}, value {} ('{}'): {:?}",
                line_number + 1,
                value_index + 1,
                value,
                e
              )
            })
          })
          .collect::<Result<Vec<isize>, String>>()?;

        Ok(Equation { result, values })
      })
      .collect::<Result<Vec<Equation>, String>>()
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    #[cfg(not(feature = "part2"))]
    let operations = [Operator::Add, Operator::Multiply];
    #[cfg(feature = "part2")]
    let operations =
      [Operator::Add, Operator::Multiply, Operator::Concatenation];

    data
      .iter()
      .filter_map(|equation| match equation.is_valid_under(&operations) {
        Ok(true) => Some(Ok(equation.result)),
        Ok(false) => None,
        Err(_) => {
          Some(Err(format!("Failed to validate equation: {:?}", equation)))
        }
      })
      .collect::<Result<Vec<isize>, String>>()
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(values) => {
        println!("tot values: {}", values.iter().sum::<isize>());
        Ok(())
      }
      Err(e) => Err(e),
    }
  }
}


#[cfg(test)]
mod tests {
  use super::{prelude::*, *};

  use crate::mock_src_provider;


  // MARK extract
  #[test]
  #[mry::lock(src_provider)] // Lock the function for mocking.
  fn test_extract() {
    let data = "190: 10 19
3267: 81 40 27";
    mock_src_provider().returns(Ok(data.to_string()));
    let result = extract().expect("Failed to extract");

    assert_eq!(result.len(), 2);
    assert_eq!(result[0], Equation { result: 190, values: vec![10, 19] });
    assert_eq!(
      result[1],
      Equation { result: 3267, values: vec![81, 40, 27] }
    );
  }

  #[test]
  #[mry::lock(src_provider)]
  fn test_extract_failure_case() {
    let data = "190: 10 19
190: 10: 19
3267: 81: 40 27";
    mock_src_provider().returns(Ok(data.to_string()));
    let result = extract();

    assert!(result.is_err());
  }

  // MARK transform
  #[cfg(all(feature = "sample", not(feature = "part2")))]
  #[test]
  fn test_transform() {
    let data = extract().expect("Failed to extract");
    let result = transform(data).expect("Failed to transform");

    assert_eq!(result.iter().sum::<isize>(), 3749);
  }

  #[cfg(all(feature = "sample", feature = "part2"))]
  #[test]
  fn test_transform_part2() {
    let data = extract().expect("Failed to extract");
    let result = transform(data).expect("Failed to transform");

    assert_eq!(result.iter().sum::<isize>(), 11387);
  }

  // MARK load
}
