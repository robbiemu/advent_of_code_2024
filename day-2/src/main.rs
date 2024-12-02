#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

type ProblemDefinition = Vec<Vec<i32>>;
type Consequent = Vec<bool>;

const MIN_BOUNDED_DIFF: i32 = 1;
const MAX_BOUNDED_DIFF: i32 = 3;


#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<ProblemDefinition, String> {
  let pd: ProblemDefinition = src_provider()?
    .lines()
    .map(|line| {
      line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
    })
    .collect();

  Ok(pd)
}

#[cfg(not(feature = "part2"))]
fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  Ok(data.iter().map(|line| is_safe(line)).collect())
}

#[cfg(feature = "part2")]
fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  Ok(
    data
      .iter()
      .map(|line| {
        is_safe(line)
          || (0..line.len()).any(|i| {
            let mut new_line = line.clone();
            new_line.remove(i);
            is_safe(&new_line)
          })
      })
      .collect(),
  )
}

fn is_safe(report: &[i32]) -> bool {
  let increasing = report.windows(2).all(|w| w[0] < w[1]);
  let decreasing = report.windows(2).all(|w| w[0] > w[1]);
  let bounded = report.windows(2).all(|w| {
    let diff = (w[1] - w[0]).abs();
    (MIN_BOUNDED_DIFF..=MAX_BOUNDED_DIFF).contains(&diff)
  });
  (increasing || decreasing) && bounded
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(consequent) => {
      let sum: usize = consequent.iter().map(|&b| b as usize).sum();
      println!("Consequent sum: {}", sum);
    }
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
    let data = extract().expect("got error on extract");

    assert_eq!(data.len(), 6);
  }

  // MARK transform
  #[cfg(not(feature = "part2"))]
  #[test]
  fn test_transform_part1() {
    let data = extract().expect("got error on extract");
    let result = transform(data).unwrap();

    assert_eq!(result, [true, false, false, false, false, true]);
  }

  #[cfg(feature = "part2")]
  #[test]
  fn test_transform_part2() {
    let data = extract().expect("got error on extract");
    let result = transform(data).unwrap();

    assert_eq!(result, [true, false, false, true, true, true]);
  }

  // MARK load
}
