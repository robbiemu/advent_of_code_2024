#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[cfg(feature = "part2")]
use std::collections::HashMap;

struct ProblemDefinition {
  left: Vec<i32>,
  right: Vec<i32>,
}
type Consequent = i32;


#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<ProblemDefinition, String> {
  let (left_values, right_values): (Vec<i32>, Vec<i32>) = src_provider()?
    .lines()
    .map(|line| {
      let mut parts = line.split_whitespace();
      let left_str = parts.next().ok_or("Missing left number")?;
      let right_str = parts.next().ok_or("Missing right number")?;
      let left: i32 = left_str
        .parse()
        .map_err(|_| "Invalid left number".to_string())?;
      let right: i32 = right_str
        .parse()
        .map_err(|_| "Invalid right number".to_string())?;
      Ok((left, right))
    })
    .collect::<Result<Vec<(i32, i32)>, String>>()?
    .into_iter()
    .unzip();

  Ok(ProblemDefinition { left: left_values, right: right_values })
}

#[cfg(not(feature = "part2"))]
fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  if data.left.len() != data.right.len() {
    return Err(
      "The left and right lists must have the same length.".to_string(),
    );
  }

  let mut sorted_left = data.left;
  let mut sorted_right = data.right;
  sorted_left.sort_unstable();
  sorted_right.sort_unstable();

  let result: i32 = sorted_left
    .iter()
    .zip(sorted_right.iter())
    .map(|(&left, &right)| (left - right).abs())
    .sum();

  Ok(result)
}
#[cfg(feature = "part2")]
fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  if data.left.len() != data.right.len() {
    return Err(
      "The left and right lists must have the same length.".to_string(),
    );
  }

  // Create a dictionary to count occurrences of each right value
  let mut right_count: HashMap<i32, i32> = HashMap::new();

  for &right in &data.right {
    *right_count.entry(right).or_insert(0) += 1;
  }

  let similarity_score: i32 = data
    .left
    .iter()
    .map(|&left| left * right_count.get(&left).unwrap_or(&0))
    .sum();

  Ok(similarity_score)
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(value) => println!("The result is: {}", value),
    Err(e) => println!("Error: {}", e),
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

  // MARK extract
  #[test]
  fn test_extract() {
    let data = extract().unwrap();
    assert_eq!(data.left, vec![3, 4, 2, 1, 3, 3]);
    assert_eq!(data.right, vec![4, 3, 5, 3, 9, 3]);
  }

  // MARK transform
  #[cfg(not(feature = "part2"))]
  #[test]
  fn test_transform_part1() {
    let data = ProblemDefinition {
      left: vec![3, 4, 2, 1, 3, 3],
      right: vec![4, 3, 5, 3, 9, 3],
    };
    let result = transform(data).unwrap();
    assert_eq!(result, 11);
  }

  #[cfg(feature = "part2")]
  #[test]
  fn test_transform_part2() {
    let data = ProblemDefinition {
      left: vec![3, 4, 2, 1, 3, 3],
      right: vec![4, 3, 5, 3, 9, 3],
    };
    let result = transform(data).unwrap();
    assert_eq!(result, 31);
  }

  // MARK load
  #[test]
  fn test_load() {
    let result = load(Ok(42));
    assert!(result.is_ok());
  }

  #[test]
  fn test_load_error() {
    let result = load(Err("Test error".to_string()));
    assert!(result.is_ok()); // load should not return an error
  }
}
