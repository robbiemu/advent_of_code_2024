mod boolean_operations;
mod model;

use crate::model::Key;


const DATA: &str = include_str!("../input.txt");

pub type ProblemDefinition = Vec<Vec<Key>>;
pub type Consequent = Vec<(Vec<Key>, String)>;


fn calculate_complexity(sequence_length: usize, code: &str) -> i64 {
  // Get the numeric part by stripping leading zeros and removing any non-digit characters
  let numeric_part: i64 = code
    .chars()
    .skip_while(|c| *c == '0') // Skip leading zeros
    .take_while(|c| c.is_ascii_digit()) // Take only the numeric part
    .collect::<String>()
    .parse()
    .unwrap_or(0);

  sequence_length as i64 * numeric_part
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
  use z3::{Config, Context};

  use super::*;
  use model::{KeypadSolver, KeypadType, Robot};

  pub fn extract() -> Result<ProblemDefinition, String> {
    Ok(
      src_provider()?
        .lines()
        .map(|line| line.chars().filter_map(Key::from_char).collect())
        .collect(),
    )
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let robots = vec![
      Robot { keypad: KeypadType::Input, id: 0 },
      Robot { keypad: KeypadType::Direction, id: 1 },
      Robot { keypad: KeypadType::Direction, id: 2 },
      Robot { keypad: KeypadType::Numeric, id: 3 },
    ];

    data
      .into_iter()
      .map(|sequence| {
        // Create a new solver for each sequence with generous step limit
        let mut solver = KeypadSolver::new(&ctx, robots.clone(), sequence, 100);

        let _ = solver.prep(None);
        solver
          .bs_minimize(None)
          .map(|solution| {
            let (sequence, code_chars) = solution.into_iter().fold(
              (Vec::new(), Vec::new()),
              |(mut seq, mut code), step| {
                if let Some(input) = Key::from_i64(step.robot_inputs[0]) {
                  seq.push(input);
                }
                if step.output >= 0 {
                  if let Some(k) = Key::from_i64(step.output) {
                    code.push(k.to_string());
                  }
                }
                (seq, code)
              },
            );

            (sequence, code_chars.join(""))
          })
          .map_err(|e| format!("Failed to solve sequence: {}", e))
      })
      .collect()
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(solutions) => {
        let mut total_complexity = 0;

        for (i, (sequence, code)) in solutions.iter().enumerate() {
          let complexity = calculate_complexity(sequence.len(), code);
          total_complexity += complexity;

          println!(
            "Solution {} (length {}, code {}, complexity {}): {:?}",
            i + 1,
            sequence.len(),
            code,
            complexity,
            sequence
          );
        }

        println!("Total complexity: {}", total_complexity);
        Ok(())
      }
      Err(e) => Err(e),
    }
  }
}


#[cfg(test)]
mod tests {
  use z3::{Config, Context};

  #[allow(unused_imports)]
  use super::{prelude::*, *};
  use crate::model::{Key, KeypadSolver, KeypadType, Robot};


  #[test]
  fn test_solver_980a_minimize() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let robots = vec![
      Robot { keypad: KeypadType::Input, id: 0 },
      Robot { keypad: KeypadType::Direction, id: 1 },
      Robot { keypad: KeypadType::Direction, id: 2 },
      Robot { keypad: KeypadType::Numeric, id: 3 },
    ];

    let target_sequence = vec![Key::Nine, Key::Eight, Key::Zero, Key::A];
    println!("\nTarget sequence: {:?}", target_sequence);

    let mut solver = KeypadSolver::new(&ctx, robots, target_sequence, 100);

    match solver.bs_minimize(None) {
      Ok(solution) => {
        println!("\nOptimal solution found with {} steps", solution.len());
        for (step_idx, step) in solution.iter().enumerate() {
          println!("\nStep {}:", step_idx);
          println!("Robot positions: {:?}", step.positions);
          println!("Robot inputs: {:?}", step.robot_inputs);
          println!("Output: {:?}", step.output);
        }

        assert_eq!(solution.len(), 60);
      }
      Err(e) => panic!("Failed to minimize sequence: {}", e),
    }
  }

  // MARK extract
  // #[mry::lock(src_provider)] // Lock the function for mocking.

  // MARK transform
  #[cfg(not(feature = "part2"))]
  #[test]
  #[mry::lock(src_provider)]
  fn test_transform() {
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));

    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("Failed to transform data");

    assert_eq!(
      result
        .iter()
        .map(|(sequence, _)| sequence.len())
        .collect::<Vec<_>>(),
      [68, 60, 68, 64, 64]
    );
  }

  // MARK load
}
