use nom::{
  bytes::complete::{tag, take_while1},
  character::complete::{line_ending, space1},
  combinator::map_res,
  multi::separated_list1,
  sequence::{preceded, separated_pair, tuple},
  IResult,
};
use std::collections::HashMap;

const DATA: &str = include_str!("../input.txt");

pub struct ProblemDefinition {
  pub initial_wires: HashMap<String, bool>,
  pub gates: Vec<(Gate, String)>,
}

pub type Consequent = HashMap<String, bool>;

#[derive(Debug, Clone)]
pub enum Gate {
  And(String, String),
  Or(String, String),
  Xor(String, String),
}

fn parse_wire(input: &str) -> IResult<&str, String> {
  map_res(take_while1(|c: char| c.is_alphanumeric()), |s: &str| {
    Ok::<_, nom::error::Error<&str>>(s.to_string())
  })(input)
}

fn parse_gate(input: &str) -> IResult<&str, (Gate, String)> {
  let (input, (left, _, op, _, right, _, _, output)) = tuple((
    parse_wire,
    space1,
    take_while1(|c: char| c.is_ascii_uppercase()),
    space1,
    parse_wire,
    space1,
    tag("->"),
    preceded(space1, parse_wire),
  ))(input)?;

  let gate = match op {
    "AND" => Gate::And(left, right),
    "OR" => Gate::Or(left, right),
    "XOR" => Gate::Xor(left, right),
    _ => unreachable!(),
  };

  Ok((input, (gate, output)))
}

fn parse_initial_wires(input: &str) -> IResult<&str, HashMap<String, bool>> {
  let (input, lines) = separated_list1(
    line_ending,
    separated_pair(parse_wire, tag(": "), parse_bool),
  )(input)?;
  Ok((input, lines.into_iter().collect()))
}

fn parse_bool(input: &str) -> IResult<&str, bool> {
  map_res(take_while1(|c: char| c == '0' || c == '1'), |s: &str| {
    Ok::<bool, nom::error::Error<&str>>(s == "1")
  })(input)
}

fn parse_gates(input: &str) -> IResult<&str, Vec<(Gate, String)>> {
  separated_list1(line_ending, parse_gate)(input)
}

fn binary_from_consequent(
  consequent: &Consequent,
) -> Result<u64, std::num::ParseIntError> {
  let mut result_vec = consequent
    .iter()
    .filter_map(|(key, value)| {
      key
        .strip_prefix('z')
        .and_then(|n| n.parse::<u32>().ok())
        .map(|index| (index, *value))
    })
    .collect::<Vec<_>>();

  result_vec.sort_by(|a, b| b.0.cmp(&a.0));

  let binary_string: String = result_vec
    .into_iter()
    .map(|(_, v)| if v { '1' } else { '0' })
    .collect();

  u64::from_str_radix(&binary_string, 2)
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
  use super::*;

  pub fn extract() -> Result<ProblemDefinition, String> {
    let data = src_provider()?;
    let sections: Vec<_> = data.split("\n\n").collect();

    if sections.len() != 2 {
      return Err("Expected exactly two sections".to_string());
    }

    let (_, initial_wires) = parse_initial_wires(sections[0].trim())
      .map_err(|e| format!("Failed to parse initial wires: {:?}", e))?;

    let (_, gates) = parse_gates(sections[1].trim())
      .map_err(|e| format!("Failed to parse gates: {:?}", e))?;

    Ok(ProblemDefinition { initial_wires, gates })
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let mut wires = data.initial_wires;
    let mut changed = true;

    while changed {
      changed = false;
      for (gate, output) in &data.gates {
        if wires.contains_key(output) {
          continue;
        }

        let result = match gate {
          Gate::And(a, b) => wires
            .get(a)
            .zip(wires.get(b))
            .map(|(&a_val, &b_val)| a_val && b_val),
          Gate::Or(a, b) => wires
            .get(a)
            .zip(wires.get(b))
            .map(|(&a_val, &b_val)| a_val || b_val),
          Gate::Xor(a, b) => wires
            .get(a)
            .zip(wires.get(b))
            .map(|(&a_val, &b_val)| a_val ^ b_val),
        };

        if let Some(value) = result {
          wires.insert(output.clone(), value);
          changed = true;
        }
      }
    }

    Ok(wires)
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    result.and_then(|wires| {
      binary_from_consequent(&wires)
        .map(|decimal| {
          println!("decimal result: {decimal}");
        })
        .map_err(|e| e.to_string())
    })
  }
}

#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::*;

  // MARK extract
  // #[mry::lock(src_provider)] // Lock the function for mocking.

  // MARK transform

  #[cfg(not(feature = "part2"))]
  #[test]
  #[mry::lock(src_provider)]
  fn test_transform() {
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));

    let data = prelude::extract().expect("Failed to extract data");
    let result = prelude::transform(data).expect("Failed to transform data");

    let mut result_vec: Vec<_> = result
      .iter()
      .filter(|(k, _)| k.starts_with('z'))
      .map(|(k, &v)| (k.clone(), v))
      .collect();
    result_vec.sort_by(|a, b| b.0.cmp(&a.0));

    let binary_string: String = result_vec
      .into_iter()
      .map(|(_, v)| if v { '1' } else { '0' })
      .collect();

    assert_eq!(binary_string, "0011111101000");
  }

  // MARK load
}
