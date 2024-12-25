use petgraph::graph::UnGraph;

mod graph;


const DATA: &str = include_str!("../input.txt");

pub struct ProblemDefinition {
  #[allow(dead_code)]
  labels: Vec<(String, String)>,
  graph: UnGraph<String, ()>,
}
#[cfg(not(feature = "part2"))]
pub type Consequent = HashSet<Vec<String>>;
#[cfg(feature = "part2")]
pub type Consequent = Vec<String>;


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
  use petgraph::graph::UnGraph;
  #[cfg(not(feature = "part2"))]
  use std::collections::HashSet;

  #[cfg(not(feature = "part2"))]
  use crate::graph::cliques_of_size;
  #[cfg(feature = "part2")]
  use crate::graph::maximum_clique;
  use crate::{src_provider, Consequent, ProblemDefinition};
  use itertools::Itertools;


  pub fn extract() -> Result<ProblemDefinition, String> {
    let labels: Vec<(String, String)> = src_provider()?
      .lines()
      .map(|line| line.split_once("-").ok_or("Split failed".to_string()))
      .map(|res| res.map(|(a, b)| (a.to_string(), b.to_string())))
      .collect::<Result<Vec<_>, _>>()?;

    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut nodes = std::collections::HashMap::new();

    for (left, right) in &labels {
      let n1 = *nodes.entry(left).or_insert(graph.add_node(left.clone()));
      let n2 = *nodes.entry(right).or_insert(graph.add_node(right.clone()));

      graph.add_edge(n1, n2, ());
    }

    Ok(ProblemDefinition { graph, labels })
  }

  #[cfg(not(feature = "part2"))]
  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let mut triangles = HashSet::new();
    for clique in cliques_of_size(&data.graph, 3) {
      let mut triangle: Vec<_> = clique
        .iter()
        .map(|&idx| data.graph[idx].to_owned())
        .collect();
      if triangle.iter().any(|label| label.starts_with('t')) {
        triangle.sort(); // Sort for consistent representation
        triangles.insert(triangle);
      }
    }

    Ok(triangles)
  }

  #[cfg(feature = "part2")]
  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    Ok(
      maximum_clique(&data.graph)
        .into_iter()
        .map(|idx| data.graph[idx].clone())
        .sorted()
        .collect(),
    )
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(triangles) => {
        #[cfg(not(feature = "part2"))]
        println!("Triangles: {:?}", triangles.len());
        #[cfg(feature = "part2")]
        println!("Password: {}", triangles.iter().join(","));
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

    assert_eq!(result.len(), 7);
  }

  #[cfg(feature = "part2")]
  #[test]
  #[mry::lock(src_provider)]
  fn test_transform() {
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));

    let data = extract().expect("failed to extract data");
    let result = transform(data).expect("failed to transform data");

    assert_eq!(result, vec!["co", "de", "ka", "ta"]);
  }

  // MARK load
}
