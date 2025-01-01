use petgraph::graph::{NodeIndex, UnGraph};


pub struct CliquesIterator<'a> {
  graph: &'a UnGraph<String, ()>,
  size: usize,
  vertices: Vec<NodeIndex>,
  stack: Vec<(Vec<NodeIndex>, usize)>, // (current_clique, next_vertex_index)
}

impl CliquesIterator<'_> {
  fn is_connected_to_all(&self, node: NodeIndex, others: &[NodeIndex]) -> bool {
    others
      .iter()
      .all(|&other| self.graph.contains_edge(node, other))
  }
}

impl Iterator for CliquesIterator<'_> {
  type Item = Vec<NodeIndex>;

  fn next(&mut self) -> Option<Self::Item> {
    // Initialize stack if empty
    if self.stack.is_empty() {
      self.stack.push((Vec::new(), 0));
    }

    while let Some((current_clique, index)) = self.stack.pop() {
      // If we've found a clique of the desired size, return it
      if current_clique.len() == self.size {
        return Some(current_clique);
      }

      // Try remaining vertices
      for i in index..self.vertices.len() {
        let vertex = self.vertices[i];

        // Check if this vertex can extend the current clique
        if self.is_connected_to_all(vertex, &current_clique) {
          let mut new_clique = current_clique.clone();
          new_clique.push(vertex);

          // Save our current state to backtrack later
          self.stack.push((current_clique.clone(), i + 1));

          // Add new state with extended clique
          self.stack.push((new_clique, i + 1));

          break;
        }
      }
    }

    None
  }
}

#[cfg(not(feature = "part2"))]
pub fn cliques_of_size(
  graph: &UnGraph<String, ()>,
  size: usize,
) -> impl Iterator<Item = Vec<NodeIndex>> + '_ {
  CliquesIterator {
    graph,
    size,
    vertices: graph.node_indices().collect(),
    stack: Vec::new(),
  }
}

#[cfg(feature = "part2")]
pub fn maximum_clique(graph: &UnGraph<String, ()>) -> Vec<NodeIndex> {
  let mut max_clique = Vec::new();
  let mut stack: Vec<(Vec<NodeIndex>, usize)> = vec![(Vec::new(), 0)];
  let vertices: Vec<NodeIndex> = graph.node_indices().collect();

  while let Some((current_clique, index)) = stack.pop() {
    // Update the maximum clique if the current clique is larger
    if current_clique.len() > max_clique.len() {
      max_clique = current_clique.clone();
    }

    // Try adding more vertices to the current clique
    (index..vertices.len()).for_each(|i| {
      let vertex = vertices[i];

      // Check if this vertex can extend the current clique
      if current_clique
        .iter()
        .all(|&other| graph.contains_edge(vertex, other))
      {
        let mut new_clique = current_clique.clone();
        new_clique.push(vertex);

        // Push the new state onto the stack
        stack.push((new_clique, i + 1));
      }
    });
  }

  max_clique
}
