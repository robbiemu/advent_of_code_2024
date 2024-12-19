use cached::proc_macro::cached;
use std::collections::HashSet;
use trie_rs::Trie;


const DATA: &str = include_str!("../input.txt");

pub struct ProblemDefinition {
  sample_space: HashSet<String>,
  onsen_designs: Vec<String>,
}
#[cfg(not(feature = "part2"))]
pub type Consequent = Vec<bool>;
#[cfg(feature = "part2")]
pub type Consequent = Vec<usize>;


#[cfg(feature = "part2")]
#[cached(key = "String", convert = r#"{ pattern.clone() }"#)]
fn satisfying_set(pattern: String, trie: &Trie<u8>) -> usize {
  if pattern.is_empty() {
    return 1;
  }

  let mut total = 0;
  let prefixes: Vec<Vec<u8>> = trie.common_prefix_search(&pattern).collect();
  for prefix in prefixes {
    let remainder = pattern[prefix.len()..].to_string();
    total += satisfying_set(remainder, trie);
  }

  total
}

#[cfg(not(feature = "part2"))]
#[cached(key = "String", convert = r#"{ design.clone() }"#)]
fn is_satisfiable(design: String, trie: &Trie<u8>) -> bool {
  if design.is_empty() {
    return true;
  }

  if trie.exact_match(&design) {
    return true;
  }

  for prefix in trie.common_prefix_search(&design) {
    let prefix: Vec<u8> = prefix;
    let remainder = design[prefix.len()..].to_string();
    if is_satisfiable(remainder, trie) {
      return true;
    }
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
  #[cfg(not(feature = "part2"))]
  use crate::is_satisfiable;
  #[cfg(feature = "part2")]
  use crate::satisfying_set;
  use crate::{src_provider, Consequent, ProblemDefinition};
  use std::collections::HashSet;
  use trie_rs::TrieBuilder;

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
    let mut builder = TrieBuilder::new();
    for sample in data.sample_space {
      builder.push(sample.as_str());
    }
    let trie = builder.build();

    #[cfg(not(feature = "part2"))]
    {
      return Ok(
        data
          .onsen_designs
          .iter()
          .map(|design| is_satisfiable(design.to_owned(), &trie))
          .collect::<Vec<_>>(),
      );
    }
    #[cfg(feature = "part2")]
    {
      Ok(
        data
          .onsen_designs
          .iter()
          .map(|design| satisfying_set(design.to_owned(), &trie))
          .collect::<Vec<_>>(),
      )
    }
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(output) => {
        #[cfg(not(feature = "part2"))]
        println!(
          "Count of satisfyable designs: {:?}",
          output.iter().map(|&b| if b { 1 } else { 0 }).sum::<u16>()
        );
        #[cfg(feature = "part2")]
        println!(
          "Count of satisfyable designs: {:?}",
          output.iter().sum::<usize>()
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

  #[cfg(feature = "part2")]
  #[test]
  #[mry::lock(src_provider)]
  fn test_transform_part2() {
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));
    let data = extract().expect("failed to extract data");
    let result = transform(data).expect("failed to transform data");

    assert_eq!(result, vec![2, 1, 4, 6, 1, 2]);
  }

  // MARK load
}
