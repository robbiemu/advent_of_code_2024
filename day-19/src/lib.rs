use std::collections::{HashSet, VecDeque};

use trie_rs::TrieBuilder;


const DATA: &str = include_str!("../input.txt");

pub struct ProblemDefinition {
  sample_space: HashSet<String>,
  onsen_designs: Vec<String>,
}
pub type Consequent = Vec<bool>;


fn is_satisfiable(design: &str, sample_space: &HashSet<String>) -> bool {
  let mut builder = TrieBuilder::new();
  for sample in sample_space {
    builder.push(sample.as_str());
  }
  let trie = builder.build();
  let mut queue = VecDeque::from([design.to_string()]);

  let mut visited = HashSet::new();
  while let Some(current_design) = queue.pop_front() {
    if visited.contains(&current_design) {
      continue;
    }
    visited.insert(current_design.clone());

    if trie.exact_match(&current_design) {
      return true;
    }

    trie
      .common_prefix_search(&current_design)
      .for_each(|prefix: String| {
        let remainder = &current_design[prefix.len()..];
        queue.push_back(remainder.to_string());
      });
  }
  false
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
  use std::collections::HashSet;

  use crate::{is_satisfiable, src_provider, Consequent, ProblemDefinition};


  pub fn extract() -> Result<ProblemDefinition, String> {
    let input = src_provider()?;
    let Some((sample_string, design_strings)) = input.split_once("\n\n") else {
      return Err("Invalid input format".to_string());
    };

    let sample_space = sample_string
      .split(", ")
      .map(String::from)
      .collect::<HashSet<_>>();
    let onsen_designs =
      design_strings.lines().map(String::from).collect::<Vec<_>>();

    Ok(ProblemDefinition { sample_space, onsen_designs })
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    Ok(
      data
        .onsen_designs
        .iter()
        .map(|design| is_satisfiable(design, &data.sample_space))
        .collect::<Vec<_>>(),
    )
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(output) => {
        println!(
          "Count of satisfyable designs: {:?}",
          output.iter().map(|&b| if b { 1 } else { 0 }).sum::<u16>()
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
  // #[mry::lock(src_provider)] // Lock the function for mocking.

  // MARK transform
  #[cfg(not(feature = "part2"))]
  #[test]
  #[mry::lock(src_provider)]
  fn test_transform() {
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));
    let data = extract().expect("failed to extract data");
    let result = transform(data).expect("failed to transform data");

    dbg!(&result);

    assert_eq!(
      result.iter().map(|&b| if b { 1 } else { 0 }).sum::<u16>(),
      6
    );
  }

  // MARK load
}
