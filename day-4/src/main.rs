#![allow(unused_assignments, dead_code)]
#[cfg(feature = "part2")]
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use vector2d::Vector2D;

#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

const DIRECTIONS: [(i32, i32); 8] = [
  (0, 1),   // right
  (0, -1),  // left
  (1, 0),   // down
  (-1, 0),  // up
  (1, 1),   // top-right
  (-1, 1),  // top-left
  (1, -1),  // bottom-right
  (-1, -1), // bottom-left
];

#[derive(Debug, Clone, Copy)]
struct HashableVector2D(Vector2D<i32>);

impl PartialEq for HashableVector2D {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl Eq for HashableVector2D {}

impl Hash for HashableVector2D {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.x.hash(state);
    self.0.y.hash(state);
  }
}

type WordCoordinates = Vec<Vector2D<i32>>;

#[cfg(feature = "part2")]
impl From<Mas> for WordCoordinates {
  fn from(mas: Mas) -> Self {
    vec![mas.m, mas.a, mas.s]
  }
}

#[cfg(feature = "part2")]
#[derive(Debug, Clone, Copy)]
struct Mas {
  m: Vector2D<i32>,
  a: Vector2D<i32>,
  s: Vector2D<i32>,
}
#[cfg(feature = "part2")]
impl From<WordCoordinates> for Mas {
  fn from(value: WordCoordinates) -> Self {
    // Ensure the vector has at least 3 elements
    assert!(
      value.len() >= 3,
      "WordCoordinates must have at least 3 elements."
    );

    Self { m: value[0], a: value[1], s: value[2] }
  }
}
#[cfg(feature = "part2")]
impl Mas {
  fn to_vec(self) -> Vec<Vector2D<i32>> {
    vec![self.m, self.a, self.s]
  }
}

type ProblemDefinition = Vec<Vec<char>>;
#[cfg(not(feature = "part2"))]
type Consequent = Vec<WordCoordinates>;
#[cfg(feature = "part2")]
type Consequent = Vec<(WordCoordinates, WordCoordinates)>;

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
  Ok(
    src_provider()?
      .lines()
      .map(|line| line.chars().map(|c| c.to_ascii_lowercase()).collect())
      .collect(),
  )
}

fn find_pattern(
  data: &ProblemDefinition,
  i: usize,
  j: usize,
  remaining_characters: &[char],
) -> Option<Vec<Vec<Vector2D<i32>>>> {
  let mut instances = Vec::new();
  for &(di, dj) in &DIRECTIONS {
    let mut coordinates = vec![Vector2D { x: j as i32, y: i as i32 }];
    for step in 1..=remaining_characters.len() {
      let ni = i as i32 + di * step as i32;
      let nj = j as i32 + dj * step as i32;
      if ni < 0
        || ni >= data.len() as i32
        || nj < 0
        || nj >= data[ni as usize].len() as i32
      {
        break;
      }
      if data[ni as usize][nj as usize] == remaining_characters[step - 1] {
        coordinates.push(Vector2D { x: nj, y: ni });
      } else {
        break;
      }
    }
    if coordinates.len() == remaining_characters.len() + 1 {
      instances.push(coordinates);
    }
  }
  if instances.is_empty() {
    None
  } else {
    Some(instances)
  }
}

#[cfg(feature = "part2")]
fn xmas_finder(
  xmas_list: Vec<WordCoordinates>,
) -> Result<Vec<(WordCoordinates, WordCoordinates)>, String> {
  let mut grouped_by_a: HashMap<HashableVector2D, Vec<WordCoordinates>> =
    HashMap::new();

  // Group instances by their 'A' coordinates
  for instance in &xmas_list {
    grouped_by_a
      .entry(HashableVector2D(Mas::from(instance.clone()).a))
      .or_default()
      .push(instance.clone());
  }

  let mut filtered_xmas = Vec::new();

  // Process each group and handle according to its size
  for (_, group) in grouped_by_a {
    match group.len() {
      4 => {
        // One of the two instances are a cross, the other a T. We'll just pick at random.
        filtered_xmas.push((group[0].to_owned(), group[1].to_owned()));
      }
      1 => {
        // Ignore groups with only 1 instance
        continue;
      }
      _ => {
        // Delegate to helper `x` for cases of 2 or 3
        let mases = group
          .iter()
          .map(|x| Mas::from(x.clone()))
          .collect::<Vec<Mas>>();

        if let Some([xmas1, xmas2]) = x(&mases) {
          filtered_xmas.push((xmas1.into(), xmas2.into()));
        }
      }
    }
  }

  if filtered_xmas.is_empty() {
    Err("No valid XMAS pairs found.".to_string())
  } else {
    Ok(filtered_xmas)
  }
}

#[cfg(feature = "part2")]
fn is_cross(xmas1: &Mas, xmas2: &Mas) -> bool {
  if xmas1.a != xmas2.a {
    return false;
  }

  let delta_m1 = xmas1.m - xmas1.a;
  let delta_s1 = xmas1.s - xmas1.a;
  let delta_m2 = xmas2.m - xmas2.a;
  let delta_s2 = xmas2.s - xmas2.a;

  // Define symmetry conditions for corners and edges
  let corner_positions = [
    Vector2D { x: -1, y: -1 },
    Vector2D { x: -1, y: 1 },
    Vector2D { x: 1, y: -1 },
    Vector2D { x: 1, y: 1 },
  ];

  let m_opposite = corner_positions.contains(&delta_m1)
    && corner_positions.contains(&delta_m2);
  let s_opposite = corner_positions.contains(&delta_s1)
    && corner_positions.contains(&delta_s2);

  m_opposite && s_opposite
}

#[cfg(feature = "part2")]
fn x(group: &[Mas]) -> Option<[Mas; 2]> {
  if group.len() == 2 {
    if is_cross(&group[0], &group[1]) {
      Some([group[0].to_owned(), group[1].to_owned()])
    } else {
      None
    }
  } else if group.len() == 3 {
    for i in 0..group.len() {
      for j in i + 1..group.len() {
        if is_cross(&group[i], &group[j]) {
          return Some([group[i].to_owned(), group[j].to_owned()]);
        }
      }
    }
    None
  } else {
    unreachable!("Invalid group length for part 2")
  }
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  let mut xmas = Vec::new();

  for (i, row) in data.iter().enumerate() {
    for (j, letter) in row.iter().enumerate() {
      #[cfg(not(feature = "part2"))]
      if *letter == 'x' {
        let Some(instances) = find_pattern(&data, i, j, &['m', 'a', 's'])
        else {
          continue;
        };
        xmas.extend(instances);
      }
      #[cfg(feature = "part2")]
      if *letter == 'm' {
        let Some(instances) = find_pattern(&data, i, j, &['a', 's']) else {
          continue;
        };
        xmas.extend(instances);
      }
    }
  }

  #[cfg(not(feature = "part2"))]
  return Ok(xmas);

  #[cfg(feature = "part2")]
  return xmas_finder(xmas);
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(xmas) => {
      println!("XMAS instances found: {:?}", xmas.len());
    }
    Err(e) => {
      println!("Error: {}", e);
    }
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

  // MARK extract_
  // #[mry::lock(src_provider)] // Lock the function for mocking.

  // MARK transform_
  #[test]
  #[cfg(all(feature = "sample", not(feature = "part2")))]
  fn test_transform() {
    let data = extract();
    assert!(data.is_ok());

    let result = transform(data.unwrap());
    match result {
      Ok(xmas) => assert_eq!(xmas.len(), 18),
      Err(e) => panic!("Error: {e}",),
    }
  }

  #[test]
  #[cfg(all(feature = "sample", feature = "part2"))]
  fn test_transform() {
    let data = extract();
    assert!(data.is_ok());

    let result = transform(data.unwrap());
    match result {
      Ok(xmas) => assert_eq!(xmas.len(), 9),
      Err(e) => panic!("Error: {e}",),
    }
  }

  // MARK load_
}
