use std::str::FromStr;
use z3::ast::Ast;
use z3::*;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[cfg(not(feature = "part2"))]
const MAX_PRESSES: i32 = 100;
#[cfg(not(feature = "part2"))]
const OFFSET: i64 = 0;

#[cfg(feature = "part2")]
const MAX_PRESSES: i64 = 10_000_000_000_000;
#[cfg(feature = "part2")]
const OFFSET: i64 = 10_000_000_000_000;

#[cfg(not(feature = "part2"))]
pub type ScaleType = i32;
#[cfg(not(feature = "part2"))]
pub type TokenCostType = u32;
#[cfg(feature = "part2")]
pub type TokenCostType = u64;

#[cfg(feature = "part2")]
pub type ScaleType = i64;
#[cfg(not(feature = "part2"))]
pub type PressType = i16;
#[cfg(feature = "part2")]
pub type PressType = i64;


#[cfg(not(feature = "part2"))]
#[derive(Debug)]
pub struct ClawMachine {
  pub a_x: i16,
  pub a_y: i16,
  pub b_x: i16,
  pub b_y: i16,
  pub prize_x: i64, // Changed to i64 to accommodate OFFSET addition
  pub prize_y: i64,
}

#[cfg(feature = "part2")]
#[derive(Debug)]
pub struct ClawMachine {
  pub a_x: i64,
  pub a_y: i64,
  pub b_x: i64,
  pub b_y: i64,
  pub prize_x: i64,
  pub prize_y: i64,
}

impl FromStr for ClawMachine {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let lines: Vec<&str> = s
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect();

    if lines.len() != 3 {
      return Err(format!(
        "Expected 3 lines per ClawMachine, found {}",
        lines.len()
      ));
    }

    #[cfg(not(feature = "part2"))]
    let button_a = lines[0]
      .strip_prefix("Button A: X+")
      .ok_or("Missing 'Button A: X+' prefix")?
      .split(", Y+")
      .map(|v| v.parse::<i16>().map_err(|e| format!("Parse error: {}", e)))
      .collect::<Result<Vec<_>, _>>()?;
    #[cfg(feature = "part2")]
    let button_a = lines[0]
      .strip_prefix("Button A: X+")
      .ok_or("Missing 'Button A: X+' prefix")?
      .split(", Y+")
      .map(|v| v.parse::<i64>().map_err(|e| format!("Parse error: {}", e)))
      .collect::<Result<Vec<_>, _>>()?;

    if button_a.len() != 2 {
      return Err(format!(
        "Expected 2 values for Button A, found {}",
        button_a.len()
      ));
    }

    #[cfg(not(feature = "part2"))]
    let button_b = lines[1]
      .strip_prefix("Button B: X+")
      .ok_or("Missing 'Button B: X+' prefix")?
      .split(", Y+")
      .map(|v| v.parse::<i16>().map_err(|e| format!("Parse error: {}", e)))
      .collect::<Result<Vec<_>, _>>()?;
    #[cfg(feature = "part2")]
    let button_b = lines[1]
      .strip_prefix("Button B: X+")
      .ok_or("Missing 'Button B: X+' prefix")?
      .split(", Y+")
      .map(|v| v.parse::<i64>().map_err(|e| format!("Parse error: {}", e)))
      .collect::<Result<Vec<_>, _>>()?;

    if button_b.len() != 2 {
      return Err(format!(
        "Expected 2 values for Button B, found {}",
        button_b.len()
      ));
    }

    #[cfg(not(feature = "part2"))]
    let prize = lines[2]
      .strip_prefix("Prize: X=")
      .ok_or("Missing 'Prize: X=' prefix")?
      .split(", Y=")
      .map(|v| v.parse::<i64>().map_err(|e| format!("Parse error: {}", e)))
      .collect::<Result<Vec<_>, _>>()?;
    #[cfg(feature = "part2")]
    let prize = lines[2]
      .strip_prefix("Prize: X=")
      .ok_or("Missing 'Prize: X=' prefix")?
      .split(", Y=")
      .map(|v| v.parse::<i64>().map_err(|e| format!("Parse error: {}", e)))
      .collect::<Result<Vec<_>, _>>()?;

    if prize.len() != 2 {
      return Err(format!(
        "Expected 2 values for Prize, found {}",
        prize.len()
      ));
    }

    Ok(ClawMachine {
      a_x: button_a[0],
      a_y: button_a[1],
      b_x: button_b[0],
      b_y: button_b[1],
      prize_x: prize[0] + OFFSET,
      prize_y: prize[1] + OFFSET,
    })
  }
}

pub struct ProblemDefinition(Vec<ClawMachine>);

impl ProblemDefinition {
  pub fn parse(input: &str) -> Result<ProblemDefinition, String> {
    // Split the input into chunks separated by empty lines
    let chunks: Vec<&str> = input
      .split("\n\n") // Split by double newlines
      .map(str::trim)
      .filter(|chunk| !chunk.is_empty())
      .collect();

    let mut machines = Vec::new();

    for chunk in chunks {
      let machine = chunk.parse::<ClawMachine>()?;
      machines.push(machine);
    }

    Ok(ProblemDefinition(machines))
  }
}

pub type Consequent = Vec<TokenCostType>;


fn solve_claw_machine(
  machine: &ClawMachine,
  max_presses: ScaleType,
) -> Option<ScaleType> {
  let cfg = Config::new();
  let ctx = Context::new(&cfg);
  let optimizer = Optimize::new(&ctx);

  let a = z3::ast::Int::new_const(&ctx, "a_presses");
  let b = z3::ast::Int::new_const(&ctx, "b_presses");

  let a_x = z3::ast::Int::from_i64(&ctx, machine.a_x as i64);
  let b_x = z3::ast::Int::from_i64(&ctx, machine.b_x as i64);
  let a_y = z3::ast::Int::from_i64(&ctx, machine.a_y as i64);
  let b_y = z3::ast::Int::from_i64(&ctx, machine.b_y as i64);

  optimizer.assert(&a.ge(&z3::ast::Int::from_i64(&ctx, 0)));
  optimizer.assert(&b.ge(&z3::ast::Int::from_i64(&ctx, 0)));
  optimizer.assert(&a.le(&z3::ast::Int::from_i64(&ctx, max_presses as i64)));
  optimizer.assert(&b.le(&z3::ast::Int::from_i64(&ctx, max_presses as i64)));

  optimizer.assert(
    &((a.clone() * &a_x + b.clone() * &b_x)
      ._eq(&z3::ast::Int::from_i64(&ctx, machine.prize_x as i64))),
  );
  optimizer.assert(
    &((a.clone() * &a_y + b.clone() * &b_y)
      ._eq(&z3::ast::Int::from_i64(&ctx, machine.prize_y as i64))),
  );

  // Minimize the token cost: 3 * a_presses + b_presses
  let objective = z3::ast::Int::add(
    &ctx,
    &[
      &z3::ast::Int::mul(&ctx, &[&z3::ast::Int::from_i64(&ctx, 3), &a]),
      &b,
    ],
  );
  optimizer.minimize(&objective);

  if optimizer.check(&[]) == z3::SatResult::Sat {
    let model = optimizer.get_model().unwrap();
    let a_val = model.eval(&a, true).unwrap().as_i64().unwrap() as ScaleType;
    let b_val = model.eval(&b, true).unwrap().as_i64().unwrap() as ScaleType;

    // Return the total token cost
    Some(3 * a_val + b_val)
  } else {
    None
  }
}

fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}


pub mod prelude {
  use crate::{
    solve_claw_machine, src_provider, Consequent, ProblemDefinition,
    TokenCostType, MAX_PRESSES,
  };

  pub fn extract() -> Result<ProblemDefinition, String> {
    ProblemDefinition::parse(&src_provider()?)
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let consequent: Consequent = data
      .0
      .iter()
      .enumerate()
      .map(|(_index, cm)| {
        //dbg!(&cm);
        match solve_claw_machine(cm, MAX_PRESSES) {
          Some(cost) => cost as TokenCostType,
          None => {
            0 // Assign 0 tokens for unsolvable machines
          }
        }
      })
      .collect();

    Ok(consequent)
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(consequent) => {
        let total_tokens: TokenCostType = consequent.iter().sum();

        println!(
          "Total tokens spent to win all possible prizes: {}",
          total_tokens
        );

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

  // MARK transform
  #[cfg(feature = "sample")]
  #[test]
  fn test_transform() {
    let data = extract().expect("failed to extract the data");
    let result = transform(data).expect("failed to transform the data");

    #[cfg(not(feature = "part2"))]
    assert_eq!(result, vec![280_u32, 0, 200, 0]);
    #[cfg(feature = "part2")]
    assert_ne!(result[1], 0_u64);
    #[cfg(feature = "part2")]
    assert_ne!(result[3], 0_u64);
  }

  // MARK load
}
