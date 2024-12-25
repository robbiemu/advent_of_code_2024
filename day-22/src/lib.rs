const DATA: &str = include_str!("../input.txt");

#[cfg(not(feature = "part2"))]
pub type ProblemDefinition = Vec<u32>;
#[cfg(feature = "part2")]
pub type ProblemDefinition = Vec<u64>;
#[cfg(not(feature = "part2"))]
pub type Consequent = Vec<usize>;
#[cfg(feature = "part2")]
pub type Consequent = ((i8, i8, i8, i8), isize);


#[cfg(not(feature = "part2"))]
fn compute_2000th_secret(initial_secret: u32) -> u64 {
  let mut secret = initial_secret as u64;
  for _ in 0..2000 {
    secret = (secret * 64) ^ secret;
    secret %= 16777216;

    secret = (secret / 32) ^ secret;
    secret %= 16777216;

    secret = (secret * 2048) ^ secret;
    secret %= 16777216;
  }
  secret
}

#[cfg(feature = "part2")]
fn compute_series(initial_secret: u64) -> Vec<u64> {
  let mut series = Vec::with_capacity(2000);
  let mut secret = initial_secret;
  for _ in 0..2000 {
    secret = (secret * 64) ^ secret;
    secret %= 16777216;

    secret = (secret / 32) ^ secret;
    secret %= 16777216;

    secret = (secret * 2048) ^ secret;
    secret %= 16777216;

    series.push(secret);
  }
  series
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
  use std::collections::HashMap;

  #[cfg(not(feature = "part2"))]
  use crate::compute_2000th_secret;
  #[cfg(feature = "part2")]
  use crate::compute_series;
  use crate::{src_provider, Consequent, ProblemDefinition};


  #[cfg(feature = "part2")]
  type Series = HashMap<(i8, i8, i8, i8), HashMap<usize, (isize, usize)>>;

  #[cfg(not(feature = "part2"))]
  pub fn extract() -> Result<ProblemDefinition, String> {
    src_provider()?
      .trim()
      .lines()
      .map(|line| line.parse::<u32>().map_err(|e| e.to_string()))
      .collect()
  }
  #[cfg(feature = "part2")]
  pub fn extract() -> Result<ProblemDefinition, String> {
    src_provider()?
      .trim()
      .lines()
      .map(|line| line.parse::<u64>().map_err(|e| e.to_string()))
      .collect()
  }

  #[cfg(not(feature = "part2"))]
  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    Ok(
      data
        .iter()
        .map(|&initial_secret| compute_2000th_secret(initial_secret) as usize)
        .collect::<Vec<_>>(),
    )
  }
  #[cfg(feature = "part2")]
  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let sequences: Vec<Vec<u64>> = data
      .iter()
      .map(|&initial| compute_series(initial))
      .collect();

    let price_sequences: Vec<Vec<i64>> = sequences
      .iter()
      .map(|seq| seq.iter().map(|&num| (num % 10) as i64).collect())
      .collect();

    let delta_sequences: Vec<Vec<i8>> = price_sequences
      .iter()
      .map(|prices| prices.windows(2).map(|w| (w[1] - w[0]) as i8).collect())
      .collect();

    let mut series: Series = HashMap::new();
    for (i, deltas) in delta_sequences.iter().enumerate() {
      for (j, window) in deltas.windows(4).enumerate() {
        let key = (window[0], window[1], window[2], window[3]);
        let value = price_sequences[i][j + 4] as isize;
        let inner_map = series.entry(key).or_default();
        inner_map.entry(i).or_insert((value, j));
      }
    }

    let mut max_sum = 0;
    let mut max_key = None;
    for (key, sequence_map) in &series {
      let sum: isize = sequence_map.values().map(|(price, _)| price).sum();
      if sum > max_sum {
        max_sum = sum;
        max_key = Some(key);
      }
    }

    if let Some(&key) = max_key {
      Ok((key, max_sum))
    } else {
      Err("No valid series found".to_string())
    }
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(consequents) => {
        // for secret in &consequents {
        //   println!("{}", secret);
        // }
        #[cfg(not(feature = "part2"))]
        println!(
          "sum: {}",
          consequents.iter().map(|&n| n as u64).sum::<u64>()
        );
        #[cfg(feature = "part2")]
        println!("sequence: {:?} sum {}", consequents.0, consequents.1);

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
  #[cfg(not(feature = "part2"))]
  #[test]
  #[mry::lock(src_provider)]
  fn test_transform() {
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));

    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("failed to transform data");

    assert_eq!(result, vec![8685429, 4700978, 15273692, 8667524]);
  }

  #[cfg(feature = "part2")]
  #[test]
  #[mry::lock(src_provider)]
  fn test_transform() {
    mock_src_provider()
      .returns(Ok(include_str!("../sample.part2.txt").to_string()));

    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("failed to transform data");

    assert_eq!(result, ((-2, 1, -1, 3), 19));
  }

  // MARK load
}
