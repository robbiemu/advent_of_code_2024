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

#[derive(Debug, Clone)]
pub struct ProblemDefinition {
  pub initial_wires: HashMap<String, bool>,
  pub gates: Vec<(Gate, String)>,
}
#[cfg(not(feature = "part2"))]
pub type Consequent = HashMap<String, bool>;
#[cfg(feature = "part2")]
pub type Consequent = Vec<String>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[cfg(not(feature = "part2"))]
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
  #[cfg(feature = "part2")]
  use std::collections::HashSet;

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

  #[cfg(not(feature = "part2"))]
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

  #[cfg(feature = "part2")]
  pub fn transform(data: ProblemDefinition) -> Result<Vec<String>, String> {
    let mut wrong_wires = HashSet::new();

    let operations: Vec<(String, String, Gate, String)> = data
      .gates
      .iter()
      .map(|(gate, output)| {
        let (op1, op2) = match gate {
          Gate::And(a, b) | Gate::Or(a, b) | Gate::Xor(a, b) => {
            (a.clone(), b.clone())
          }
        };
        (op1, op2, gate.clone(), output.clone())
      })
      .collect();

    let is_simple_adder = operations.iter().all(|(op1, op2, gate, _)| {
      matches!(gate, Gate::And(_, _))
        && op1.starts_with('x')
        && op2.starts_with('y')
    });
    if is_simple_adder {
      // Collect output wires that don't follow x{n}+y{n} → z{n} pattern
      operations
        .iter()
        .filter_map(|(op1, op2, _, res)| {
          Some((
            op1.strip_prefix('x').and_then(|s| s.parse::<u32>().ok())?,
            op2.strip_prefix('y').and_then(|s| s.parse::<u32>().ok())?,
            res,
          ))
        })
        .filter(|(x, y, _)| x == y)
        .filter(|(idx, _, res)| &format!("z{:02}", idx) != *res)
        .for_each(|(_, _, res)| {
          wrong_wires.insert(res.clone());
        });
    } else {
      // find how many registers we need to check
      let last_register = data
        .gates
        .iter()
        .map(|(_, output)| output)
        .filter_map(|output| {
          output
            .strip_prefix('z')
            .map(|_| output)
            .and_then(|o| o.strip_prefix('z'))
            .and_then(|s| s.parse::<u32>().ok())
            .map(|num| (num, output))
        })
        .fold(
          ("z00".to_string(), 0),
          |(acc_str, acc_num), (num, output)| {
            if num > acc_num {
              (output.clone(), num)
            } else {
              (acc_str, acc_num)
            }
          },
        )
        .0;

      for (op1, op2, op, res) in &operations {
        // the interim z-wires that aren't produced by XOR gates
        if res.starts_with('z')
          && !matches!(op, Gate::Xor(_, _))
          && res != &last_register
        {
          wrong_wires.insert(res.clone());
        }

        match op {
          Gate::Xor(_, _) => {
            // Combine both XOR conditions in one place
            if !res.starts_with(['x', 'y', 'z'])
              && !op1.starts_with(['x', 'y', 'z'])
              && !op2.starts_with(['x', 'y', 'z'])
            {
              wrong_wires.insert(res.clone());
            }
            // Check XOR-OR interactions
            for (subop1, subop2, subop, _subres) in &operations {
              if (res == subop1 || res == subop2)
                && matches!(subop, Gate::Or(_, _))
              {
                wrong_wires.insert(res.clone());
              }
            }
          }
          Gate::And(_, _) => {
            if op1 != "x00" && op2 != "x00" {
              for (subop1, subop2, subop, _subres) in &operations {
                if (res == subop1 || res == subop2)
                  && !matches!(subop, Gate::Or(_, _))
                {
                  wrong_wires.insert(res.clone());
                }
              }
            }
          }
          _ => {}
        }
      }
    }

    let mut result: Vec<String> = wrong_wires.into_iter().collect();
    result.sort();

    Ok(result)
  }

  #[cfg(not(feature = "part2"))]
  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    result.and_then(|wires| {
      binary_from_consequent(&wires)
        .map(|decimal| {
          println!("decimal result: {decimal}");
        })
        .map_err(|e| e.to_string())
    })
  }
  #[cfg(feature = "part2")]
  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(swaps) => {
        let mut result = swaps.join(",");
        result.push('\n');
        println!("Swaps: {}", result);
        Ok(())
      }
      Err(e) => Err(e),
    }
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

  #[cfg(feature = "part2")]
  #[test]
  #[mry::lock(src_provider)]
  fn test_transform() {
    mock_src_provider()
      .returns(Ok(include_str!("../sample.part2.txt").to_string()));

    let data = prelude::extract().expect("Failed to extract data");
    let result = prelude::transform(data).expect("Failed to transform data");

    assert_eq!(result, vec!["z00", "z01", "z02", "z05"]);
  }

  // MARK load
}
