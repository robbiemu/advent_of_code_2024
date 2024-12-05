#[cfg(feature = "part2")]
use nom::combinator::map;
use nom::{
  bytes::complete::tag,
  character::complete::{char, digit1},
  combinator::map_res,
  multi::separated_list1,
  sequence::{delimited, preceded},
  IResult,
};

#[cfg(all(feature = "sample", not(feature = "part2")))]
const DATA: &str = include_str!("../sample.txt");
#[cfg(all(feature = "sample", feature = "part2"))]
const DATA: &str = include_str!("../sample.part2.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

type ProblemDefinition = Vec<Vec<Action>>;
type Consequent = isize;

#[derive(Debug, PartialEq, Eq)]
enum Action {
  Mul((isize, isize)),
  #[cfg(feature = "part2")]
  Do,
  #[cfg(feature = "part2")]
  Dont,
}

impl Action {
  fn apply(&mut self) -> Option<isize> {
    match self {
      Action::Mul((x, y)) => Some(*x * *y),
      #[cfg(feature = "part2")]
      _ => None, // no-op
    }
  }
}

fn parse_number(input: &str) -> IResult<&str, isize> {
  map_res(digit1, |s: &str| s.parse::<isize>())(input)
}

fn parse_mul(input: &str) -> IResult<&str, Action> {
  preceded(
    tag("mul"),
    delimited(
      char('('), // Match opening parenthesis
      map_res(
        separated_list1(
          char(','),
          parse_number, // Match digits only
        ),
        |numbers: Vec<isize>| {
          if numbers.len() == 2 {
            Ok(Action::Mul((numbers[0], numbers[1])))
          } else {
            Err("Invalid number of arguments".to_string())
          }
        },
      ),
      char(')'), // Match closing parenthesis
    ),
  )(input)
}

#[cfg(feature = "part2")]
fn parse_do(input: &str) -> IResult<&str, Action> {
  map(tag("do()"), |_| Action::Do)(input)
}

#[cfg(feature = "part2")]
fn parse_dont(input: &str) -> IResult<&str, Action> {
  map(tag("don't()"), |_| Action::Dont)(input)
}

#[cfg(not(feature = "part2"))]
fn parse_actions(input: &str) -> IResult<&str, Vec<Action>> {
  let mut actions = Vec::new();
  let mut rest = input.trim();
  while let Some(index) = rest.find("mul") {
    rest = &rest[index..];
    match parse_mul(rest) {
      Ok((new_rest, action)) => {
        actions.push(action);
        rest = new_rest;
      }
      Err(_) => {
        rest = &rest[3..]; // Skip 'mul' to avoid infinite loop
      }
    }
  }
  Ok((rest, actions))
}

#[cfg(feature = "part2")]
fn parse_actions(input: &str) -> IResult<&str, Vec<Action>> {
  let mut actions = Vec::new();
  let mut rest = input.trim();

  while !rest.is_empty() {
    if let Ok((new_rest, action)) = parse_mul(rest) {
      actions.push(action);
      rest = new_rest;
    } else if let Ok((new_rest, action)) = parse_do(rest) {
      actions.push(action);
      rest = new_rest;
    } else if let Ok((new_rest, action)) = parse_dont(rest) {
      actions.push(action);
      rest = new_rest;
    } else {
      // Skip one character to avoid infinite loop on unexpected input
      rest = &rest[1..];
    }
  }

  Ok((rest, actions))
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

fn extract() -> Result<ProblemDefinition, String> {
  let input = src_provider()?;
  let mut lines_actions = Vec::new();
  for line in input.lines() {
    let (_, actions) =
      parse_actions(line).unwrap_or_else(|_| (line, Vec::new()));
    lines_actions.push(actions);
  }
  Ok(lines_actions)
}

fn transform(mut data: ProblemDefinition) -> Result<Consequent, String> {
  #[cfg(feature = "part2")]
  let mut mul_enabled = true; // Initially enabled

  Ok(
    data
      .iter_mut()
      .map(|actions| {
        #[cfg(not(feature = "part2"))]
        {
          actions
            .iter_mut()
            .map(|action| action.apply().unwrap_or(0))
            .sum::<isize>()
        }

        #[cfg(feature = "part2")]
        {
          actions
            .iter_mut()
            .filter_map(|action| match action {
              Action::Mul(_) => {
                if mul_enabled {
                  action.apply()
                } else {
                  None
                }
              }
              Action::Do => {
                mul_enabled = true;
                None
              }
              Action::Dont => {
                mul_enabled = false;
                None
              }
            })
            .sum::<isize>()
        }
      })
      .sum(),
  )
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(value) => println!("Success! value: {value}"),
    Err(e) => println!("Error: {:?}", e),
  }

  Ok(())
}

fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[mry::lock(src_provider)] // Lock the function for mocking.
  #[test]
  fn test_extract() {
    // Mock data
    const TEST_DATA: &str = "
    +mul(1,2)mul(3,4)
    +mul(mul(5,6)mul(8,+9)-mul(1,mul(7,8)
    +no_mul(9,10)
    ";
    mock_src_provider().returns(Ok(TEST_DATA.to_string()));

    // Expected result
    let expected: ProblemDefinition = vec![
      vec![],
      vec![Action::Mul((1, 2)), Action::Mul((3, 4))],
      vec![Action::Mul((5, 6)), Action::Mul((7, 8))],
      vec![Action::Mul((9, 10))],
      vec![],
    ];

    // Run the extract function
    let result = extract().expect("get an error on extract");

    // Assert the result
    assert_eq!(result, expected);
  }

  #[test]
  fn test_transform() {
    let data = vec![vec![
      Action::Mul((2, 4)),
      Action::Mul((5, 5)),
      Action::Mul((11, 8)),
      Action::Mul((8, 5)),
    ]];

    let result = transform(data).expect("got an error in transform");
    assert_eq!(result, 2 * 4 + 5 * 5 + 11 * 8 + 8 * 5);
  }

  #[cfg(all(feature = "sample", not(feature = "part2")))]
  #[test]
  fn test_transform_sample() {
    // sample.txt data: xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

    let data = extract().expect("Got an error in extract");
    let result = transform(data).expect("Got an error in transform");
    assert_eq!(result, 2 * 4 + 5 * 5 + 11 * 8 + 8 * 5);
  }
  #[cfg(all(feature = "sample", feature = "part2"))]
  #[test]
  fn test_transform_sample() {
    // sample.part2.txt data: xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))

    let data = extract().expect("Got an error in extract");
    let result = transform(data).expect("Got an error in transform");
    assert_eq!(result, 2 * 4 + 8 * 5);
  }
}
