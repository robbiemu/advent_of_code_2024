use nom::{
  branch::alt,
  character::complete::{char, line_ending},
  multi::{many1, many_m_n},
  sequence::tuple,
  IResult,
};


const DATA: &str = include_str!("../input.txt");

#[derive(Debug, Clone, PartialEq)]
pub struct Lock {
  heights: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Key {
  heights: Vec<usize>,
}

pub struct ProblemDefinition {
  locks: Vec<Lock>,
  keys: Vec<Key>,
}
pub type Consequent = Vec<(Key, Lock)>;


fn key_fits_lock(lock: &Lock, key: &Key) -> bool {
  if lock.heights.len() != key.heights.len() {
    return false;
  }

  for (lock_height, key_height) in lock.heights.iter().zip(key.heights.iter()) {
    if lock_height + key_height > 5 {
      return false;
    }
  }

  true
}

fn parse_row(input: &str) -> IResult<&str, String> {
  let (input, row) = many1(alt((char('#'), char('.'))))(input)?;
  Ok((input, row.into_iter().collect()))
}

fn parse_lock(input: &str) -> IResult<&str, Lock> {
  // First get all rows
  let (input, mut rows) =
    many_m_n(6, 6, tuple((parse_row, line_ending)))(input)?;
  let (input, last_row) = parse_row(input)?;
  rows.push((last_row.clone(), "\n"));

  // Convert from rows to column heights
  let mut heights = Vec::new();
  for col in 0..5 {
    let mut height = 7; // Start from max height (7 rows)
    for (i, row) in rows.iter().enumerate() {
      if row.0.chars().nth(col).unwrap() == '.' {
        height = i - 1; // First '.' marks the height
        break;
      }
    }
    heights.push(height);
  }

  Ok((input, Lock { heights }))
}

fn parse_key(input: &str) -> IResult<&str, Key> {
  // First get all rows
  let (input, mut rows) =
    many_m_n(6, 6, tuple((parse_row, line_ending)))(input)?;
  let (input, last_row) = parse_row(input)?;
  rows.push((last_row.clone(), "\n"));

  // Convert from rows to column heights
  let mut heights = Vec::new();
  for col in 0..5 {
    let mut height = 0;
    // Start from bottom row (index 6) and work up
    for i in (0..6).rev() {
      let row = &rows[i].0;
      if row.chars().nth(col).unwrap() == '#' {
        height += 1;
      } else {
        break;
      }
    }
    heights.push(height);
  }

  Ok((input, Key { heights }))
}


fn parse(input: &str) -> IResult<&str, ProblemDefinition> {
  let mut input = input;
  let mut locks = vec![];
  let mut keys = vec![];

  while !input.trim().is_empty() {
    let lines: Vec<&str> = input.lines().take(7).collect();
    if lines.is_empty() {
      break;
    }

    // Check if it's a lock (all # at top) or key (all # at bottom)
    if lines[0].chars().all(|c| c == '#') {
      if let Ok((remaining_input, lock)) = parse_lock(input) {
        locks.push(lock);
        input = remaining_input;
      }
    } else if lines
      .get(6)
      .map_or(false, |last_line| last_line.chars().all(|c| c == '#'))
    {
      if let Ok((remaining_input, key)) = parse_key(input) {
        keys.push(key);
        input = remaining_input;
      }
    }

    let (remaining_input, _) = many_m_n(0, 1, line_ending)(input)?;
    input = remaining_input;
  }

  Ok((input, ProblemDefinition { locks, keys }))
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
  //#[cfg(not(feature = "part2"))]
  use crate::key_fits_lock;
  use crate::{parse, src_provider, Consequent, ProblemDefinition};

  pub fn extract() -> Result<ProblemDefinition, String> {
    let (_, pd) = parse(&src_provider()?).map_err(|e| {
      dbg!(e);

      "Error parsing input"
    })?;

    Ok(pd)
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let mut matches = vec![];
    for lock in data.locks.iter() {
      for key in data.keys.iter() {
        if key_fits_lock(lock, key) {
          matches.push((key.to_owned(), lock.to_owned()));
        }
      }
    }

    Ok(matches)
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(consequent) => {
        println!("Keys fitting locks: {}", consequent.len());

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
  #[test]
  fn test_parsing() {
    let input = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"#;

    let result = parse(input).unwrap();
    assert_eq!(result.1.locks.len(), 2); // Adjust based on expected lock counts
    assert_eq!(result.1.keys.len(), 3); // Adjust based on expected key counts
  }

  // MARK transform
  #[cfg(not(feature = "part2"))]
  #[test]
  #[mry::lock(src_provider)]
  fn test_transform() {
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));

    println!("Running test_transform: extract");
    let input = extract().expect("Failed to extract data");
    println!("Running test_transform: transform");
    let result = transform(input).expect("Failed to transform data");

    assert_eq!(result.len(), 3); // Adjust based on expected matches
  }

  // MARK load
}
