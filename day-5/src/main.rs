#[cfg(feature = "part2")]
use std::collections::{HashMap, HashSet, VecDeque};
use std::num::ParseIntError;

#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
struct Rule {
  precedent: i32,
  consequent: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ProblemDefinition {
  rules: Vec<Rule>,
  updates: Vec<Vec<i32>>,
}
type Consequent = Vec<Vec<i32>>;


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
  match src_provider()?.split_once("\n\n") {
    Some((rules, updates)) => Ok(ProblemDefinition {
      rules: rules
        .lines()
        .map(|line| {
          let Some((precedent, consequent)) = line.split_once("|") else {
            return Err(format!("invalid rule format: {line}"));
          };
          Ok(Rule {
            precedent: precedent
              .parse()
              .map_err(|e: ParseIntError| e.to_string())?,
            consequent: consequent
              .parse()
              .map_err(|e: ParseIntError| e.to_string())?,
          })
        })
        .collect::<Result<Vec<_>, String>>()?,
      updates: updates
        .lines()
        .map(|line| {
          line
            .split(",")
            .map(|num| num.parse().map_err(|e: ParseIntError| e.to_string()))
            .collect::<Result<Vec<i32>, String>>()
        })
        .collect::<Result<Vec<_>, String>>()?,
    }),
    None => {
      Err("invalid input, did not split into rules and updates.".to_owned())
    }
  }
}

fn ensure_rule(rule: &Rule, update: &[i32]) -> bool {
  let Rule { precedent, consequent } = rule;

  if !update.contains(precedent) || !update.contains(consequent) {
    return true;
  }

  let index_precedent = match update.iter().position(|x| x == precedent) {
    Some(index) => index,
    None => unreachable!(),
  };

  let index_consequent = match update.iter().position(|x| x == consequent) {
    Some(index) => index,
    None => unreachable!(),
  };

  index_precedent < index_consequent
}

#[cfg(feature = "part2")]
fn ordered(rules: &[Rule], update: &[i32]) -> Result<Vec<i32>, String> {
  let applicable_rules: Vec<&Rule> = rules
    .iter()
    .filter(|r| update.contains(&r.precedent) && update.contains(&r.consequent))
    .collect();

  // Build the graph and incoming degrees using applicable rules
  let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
  let mut in_degrees: HashMap<i32, usize> = HashMap::new();

  let ruled_elements: HashSet<i32> = applicable_rules
    .iter()
    .flat_map(|r| vec![r.precedent, r.consequent])
    .collect();

  for &elem in &ruled_elements {
    in_degrees.entry(elem).or_insert(0);
  }

  for rule in applicable_rules {
    graph
      .entry(rule.precedent)
      .or_default()
      .push(rule.consequent);
    *in_degrees.entry(rule.consequent).or_default() += 1;
  }

  // Precompute positions to avoid repeated position lookups
  let position_map: HashMap<i32, usize> = update
    .iter()
    .enumerate()
    .filter(|&(_, &x)| ruled_elements.contains(&x))
    .map(|(i, &x)| (x, i))
    .collect();

  // Initialize queue with nodes having zero incoming degrees, in the order they appear in the update
  let mut queue: VecDeque<i32> = in_degrees
    .iter()
    .filter_map(|(&node, &deg)| if deg == 0 { Some(node) } else { None })
    .collect();

  // Sort queue based on their positions in the update to maintain stability
  let mut queue_vec: Vec<i32> = queue.iter().cloned().collect();
  queue_vec.sort_by_key(|&node| {
    position_map.get(&node).copied().unwrap_or(usize::MAX)
  });
  queue = VecDeque::from(queue_vec);

  let mut sorted: Vec<i32> = Vec::new();
  while let Some(node) = queue.pop_front() {
    sorted.push(node);
    if let Some(neighbors) = graph.get(&node) {
      for &neighbor in neighbors {
        if let Some(degree) = in_degrees.get_mut(&neighbor) {
          if *degree > 0 {
            *degree -= 1;
            if *degree == 0 {
              queue.push_back(neighbor);
            }
          }
        } else {
          return Err(format!("Neighbor {} not found in in_degrees", neighbor));
        }
      }
      // Re-sort the queue to maintain order
      let mut queue_vec: Vec<i32> = queue.iter().cloned().collect();
      queue_vec.sort_by_key(|&node| {
        position_map.get(&node).copied().unwrap_or(usize::MAX)
      });
      queue = VecDeque::from(queue_vec);
    }
  }

  if sorted.len() != ruled_elements.len() {
    return Err("Cannot satisfy all rules due to cycles.".to_string());
  }

  // Reconstruct the update
  let mut sorted_iter = sorted.iter();
  let mut result = Vec::new();
  for &elem in update {
    if ruled_elements.contains(&elem) {
      match sorted_iter.next() {
        Some(&next_sorted) => result.push(next_sorted),
        None => {
          return Err(
            "Mismatch between sorted elements and update.".to_string(),
          )
        }
      }
    } else {
      result.push(elem);
    }
  }

  Ok(result)
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  #[cfg(not(feature = "part2"))]
  return Ok(
    data
      .updates
      .iter()
      .filter(|update| data.rules.iter().all(|r| ensure_rule(r, update)))
      .cloned()
      .collect(),
  );

  #[cfg(feature = "part2")]
  {
    let mut results = Vec::new();
    for update in &data.updates {
      if data.rules.iter().all(|r| ensure_rule(r, update)) {
        continue;
      }
      match ordered(&data.rules, update) {
        Ok(ordered_update) => results.push(ordered_update),
        Err(e) => {
          return Err(format!("Failed to order update {:?}: {}", update, e))
        }
      }
    }

    Ok(results)
  }
}

fn get_middle(numbers: &[i32]) -> i32 {
  let len = numbers.len();
  numbers[len / 2]
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(consequent) => println!(
      "Consequent updates: {:?}",
      consequent
        .iter()
        .map(|update| get_middle(update))
        .sum::<i32>()
    ),
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

  use crate::src_provider;

  // MARK extract
  #[mry::lock(src_provider)] // Lock the function for mocking.
  #[test]
  fn test_extract() -> Result<(), String> {
    let data: &str = "75|13
53|13

75,47,61,53,29
97,61,53,29,13";
    mock_src_provider().returns(Ok(data.to_string()));
    let expected = ProblemDefinition {
      rules: vec![
        Rule { precedent: 75, consequent: 13 },
        Rule { precedent: 53, consequent: 13 },
      ],
      updates: vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13]],
    };
    assert_eq!(extract()?, expected);
    Ok(())
  }

  // MARK transform
  #[cfg(feature = "sample")]
  #[test]
  fn test_transform() {
    let result = extract().expect("extract failed");
    #[cfg(not(feature = "part2"))]
    let expected: Vec<Vec<i32>> = vec![
      vec![75, 47, 61, 53, 29],
      vec![97, 61, 53, 29, 13],
      vec![75, 29, 13],
    ];
    #[cfg(feature = "part2")]
    let expected: Vec<Vec<i32>> = vec![
      vec![97, 75, 47, 61, 53],
      vec![61, 29, 13],
      vec![97, 75, 47, 29, 13],
    ];

    let received = transform(result).expect("transform failed");
    assert_eq!(received, expected);
  }

  // MARK load
}
