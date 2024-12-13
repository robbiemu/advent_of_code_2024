use game_grid::{Grid, GridCell, GridPosition, ParseCellError};
#[cfg(feature = "part2")]
use std::collections::HashSet;
use std::fmt::Debug;
use std::{cmp::Ordering, collections::HashMap};

#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

const CARDINAL_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)]; // Right, Down, Left, Up
#[cfg(feature = "part2")]
const EIGHT_CONNECTED_DIRECTIONS: [(isize, isize); 8] = [
  (-1, -1),
  (-1, 0),
  (-1, 1),
  (0, -1),
  (0, 1),
  (1, -1),
  (1, 0),
  (1, 1),
];

#[derive(GridCell, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
  #[cell('A'..='Z')]
  Plant(char),
}

impl Default for Cell {
  fn default() -> Self {
    Cell::Plant('.')
  }
}

#[derive(
  GridPosition, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  fn to_index(self, cols: usize) -> usize {
    self.y as usize * cols + self.x as usize
  }
}

pub struct WrappedGrid<'a>(&'a Grid<Cell>);

impl<'a> WrappedGrid<'a> {
  fn neighbors<'b>(
    &'b self,
    point: &'a Point,
  ) -> impl Iterator<Item = Point> + 'b {
    CARDINAL_DIRECTIONS.iter().filter_map(move |(dx, dy)| {
      let p = Point { x: point.x + dx, y: point.y + dy };
      self.0.is_in_bounds(p).then_some(p)
    })
  }
}

#[cfg(feature = "part2")]
struct VisitedTracker<'a> {
  global_visited: &'a mut [Vec<bool>],
  local_visited: HashSet<(usize, usize)>,
}
#[cfg(feature = "part2")]
impl<'a> VisitedTracker<'a> {
  fn new(global_visited: &'a mut [Vec<bool>]) -> Self {
    VisitedTracker { global_visited, local_visited: HashSet::new() }
  }

  fn mark(&mut self, pos: (usize, usize)) {
    self.global_visited[pos.1][pos.0] = true;
    self.local_visited.insert(pos);
  }

  fn unmark(&mut self, pos: (usize, usize)) {
    // Remove from global but retain local tracking
    self.global_visited[pos.1][pos.0] = false;
  }

  fn is_visited(&self, pos: (usize, usize)) -> bool {
    self.global_visited[pos.1][pos.0] || self.local_visited.contains(&pos)
  }
}

#[derive(PartialEq, Eq)]
pub struct Garden {
  label: char,
  area: usize,
  perimeter: usize,
  #[cfg(feature = "part2")]
  points: Vec<Point>,
}

impl Debug for Garden {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}({},{})", self.label, self.area, self.perimeter)
  }
}

#[cfg(feature = "part2")]
impl Garden {
  fn wrap_with_zeros(&self, grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    // Get the number of rows and columns in the original grid
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Create a new grid with an extra row and column on each side, initialized with 0s
    let mut wrapped_grid = vec![vec![0; cols + 2]; rows + 2];

    // Copy the original grid into the center of the new grid
    for i in 0..rows {
      for j in 0..cols {
        wrapped_grid[i + 1][j + 1] = grid[i][j];
      }
    }

    wrapped_grid
  }

  fn expand_with_distinct_spaces(&self, input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let original_rows = input.len();
    let original_cols = input[0].len();

    // Calculate the new dimensions
    let new_rows = original_rows + (original_rows - 1) + 2;
    let new_cols = original_cols + (original_cols - 1) + 2;

    // Initialize the expanded matrix with 3s (placeholders)
    let mut output = vec![vec![3; new_cols]; new_rows];

    // Place original values and fill in-between spaces
    for (i, row) in input.iter().enumerate() {
      for (j, &value) in row.iter().enumerate() {
        let new_i = i * 2 + 1;
        let new_j = j * 2 + 1;

        output[new_i][new_j] = value;

        if j > 0 && value == 1 && input[i][j - 1] == 1 {
          output[new_i][new_j - 1] = 1;
        }

        if i > 0 && value == 1 && input[i - 1][j] == 1 {
          output[new_i - 1][new_j] = 1;
        }
      }
    }

    // Wrap with zeros to define a clear boundary
    output = self.wrap_with_zeros(&output);

    let new_rows = output.len();
    let new_cols = output[0].len();

    // Process the remaining `3`s
    for i in 0..new_rows {
      for j in 0..new_cols {
        if output[i][j] == 3 {
          // Check 8-adjacency for `1`
          let is_adjacent_to_one =
            EIGHT_CONNECTED_DIRECTIONS.iter().any(|&(di, dj)| {
              let ni = i as isize + di;
              let nj = j as isize + dj;
              ni >= 0
                && nj >= 0
                && (ni as usize) < new_rows
                && (nj as usize) < new_cols
                && output[ni as usize][nj as usize] == 1
            });

          // Check 8-adjacency for edge or `0`
          let is_adjacent_to_edge_or_zero =
            EIGHT_CONNECTED_DIRECTIONS.iter().any(|&(di, dj)| {
              let ni = i as isize + di;
              let nj = j as isize + dj;
              ni < 0
                || nj < 0
                || (ni as usize) >= new_rows
                || (nj as usize) >= new_cols
                || (ni >= 0
                  && nj >= 0
                  && (ni as usize) < new_rows
                  && (nj as usize) < new_cols
                  && output[ni as usize][nj as usize] == 0)
            });

          if is_adjacent_to_one && is_adjacent_to_edge_or_zero {
            // Now do a stricter 4-directional check.
            let is_four_adjacent_to_one_or_zero =
              CARDINAL_DIRECTIONS.iter().any(|&(di, dj)| {
                let ni = i as isize + di as isize;
                let nj = j as isize + dj as isize;
                ni >= 0
                  && nj >= 0
                  && (ni as usize) < new_rows
                  && (nj as usize) < new_cols
                  && (output[ni as usize][nj as usize] == 1
                    || output[ni as usize][nj as usize] == 0)
              });

            if is_four_adjacent_to_one_or_zero {
              // Proper adjacency: mark as `2`
              output[i][j] = 2;
            } else {
              // Only diagonal adjacency: mark as `3`
              output[i][j] = 3;
            }
          } else {
            // Doesn't qualify for `2` or `3`: mark as `0`
            output[i][j] = 0;
          }
        }
      }
    }

    output
  }

  fn expand_to_grid(&self) -> Vec<Vec<u8>> {
    // Find the bounding box of the region
    let min_x = self.points.iter().map(|p| p.x).min().unwrap_or(0);
    let max_x = self.points.iter().map(|p| p.x).max().unwrap_or(0);
    let min_y = self.points.iter().map(|p| p.y).min().unwrap_or(0);
    let max_y = self.points.iter().map(|p| p.y).max().unwrap_or(0);
    // Create the binary grid for the region
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut binary_grid = vec![vec![0; width]; height];

    for point in &self.points {
      let x = (point.x - min_x) as usize;
      let y = (point.y - min_y) as usize;
      binary_grid[y][x] = 1;
    }

    binary_grid
  }

  fn trace_and_count_perimeter(
    &self,
    grid: &[Vec<u8>],
    visited: &mut [Vec<bool>],
    start: (usize, usize),
  ) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut path = Vec::new();
    let mut current_pos = start;
    let mut current_dir = 0;
    let mut first_move = true;

    // Initialize the visited tracker with global state
    let mut tracker = VisitedTracker::new(visited);

    // Track visited `3`s and their indices in the path
    let mut visited_threes: HashMap<(usize, usize), usize> = HashMap::new();

    while !tracker.is_visited(current_pos) {
      if grid[current_pos.1][current_pos.0] != 3 {
        tracker.mark(current_pos);
      }
      path.push(current_pos);

      let mut found_next = false;

      for i in 0..CARDINAL_DIRECTIONS.len() {
        let dir_idx = (current_dir + i) % CARDINAL_DIRECTIONS.len();
        let (dx, dy) = CARDINAL_DIRECTIONS[dir_idx];
        let nx = current_pos.0 as isize + dx as isize;
        let ny = current_pos.1 as isize + dy as isize;

        if nx >= 0 && ny >= 0 && (nx as usize) < cols && (ny as usize) < rows {
          let neighbor_val = grid[ny as usize][nx as usize];

          if neighbor_val == 2
            && !tracker.is_visited((nx as usize, ny as usize))
          {
            // Continue to the next `2`
            current_dir = dir_idx;
            current_pos = (nx as usize, ny as usize);
            found_next = true;
            break;
          } else if neighbor_val == 3 {
            let neighbor_pos = (nx as usize, ny as usize);
            #[cfg(any(test, feature = "debug"))]
            println!("Neighbor ({}, {}) is a 3", ny, nx);

            if let Some(&prev_index) = visited_threes.get(&neighbor_pos) {
              // Backtrack to the previous occurrence of this `3`
              #[cfg(any(test, feature = "debug"))]
              println!(
                "Backtracking: removing loop from path at position {:?}",
                neighbor_pos
              );

              // Unmark all positions being removed
              for pos in path.drain(prev_index + 1..) {
                tracker.unmark(pos);
              }

              // Reset traversal from this `3`
              current_pos = neighbor_pos;
              found_next = true;
              break;
            } else {
              // Mark this `3` as visited and continue
              visited_threes.insert(neighbor_pos, path.len() - 1);
              current_dir = dir_idx;
              current_pos = neighbor_pos;
              found_next = true;
              break;
            }
          }
        }
      }

      if !found_next {
        // No valid move found; stop traversal
        break;
      }

      if current_pos == start && !first_move {
        // Break if we've completed a loop and it's not the first move
        break;
      }

      first_move = false;
    }

    // Compute sides from the constructed path
    if path.len() > 2 {
      self.compute_sides_from_path(&path)
    } else {
      None
    }
  }

  fn compute_sides_from_path(&self, path: &[(usize, usize)]) -> Option<usize> {
    #[cfg(any(test, feature = "debug"))]
    println!("{:?}", path);
    if path.len() <= 2 {
      return None;
    }
    let mut edges = vec![];
    let mut sides = 0;
    for i in 0..path.len() {
      let curr = (
        path[(i + 1) % path.len()].0 as isize - path[i].0 as isize,
        path[(i + 1) % path.len()].1 as isize - path[i].1 as isize,
      );
      let prev = (
        path[i].0 as isize - path[(i + path.len() - 1) % path.len()].0 as isize,
        path[i].1 as isize - path[(i + path.len() - 1) % path.len()].1 as isize,
      );
      if curr != prev {
        edges.push(path[i]);
        sides += 1;
      }
    }

    #[cfg(any(test, feature = "debug"))]
    {
      for pair in edges
        .iter()
        .chain(edges.iter().take(1))
        .collect::<Vec<_>>()
        .windows(2)
      {
        println!("Edge detected: {:?}->{:?}", pair[0], pair[1]);
      }
      println!("Edges: {}, sides: {}", edges.len(), sides);
    }
    Some(sides)
  }

  fn calculate_sides(&self) -> usize {
    let binary_grid = self.expand_to_grid();
    let expanded_grid = self.expand_with_distinct_spaces(binary_grid.clone());

    #[cfg(any(test, feature = "debug"))]
    {
      println!("calculating sides of {}", self.label);
      println!("Binary Grid:");
      for row in binary_grid.iter() {
        println!("{:?}", row);
      }
      println!("Expanded Grid:");
      for row in expanded_grid.iter() {
        println!("{:?}", row);
      }
    }
    let rows = expanded_grid.len();
    let cols = expanded_grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut total_sides = 0;

    // Find perimeters
    for y in 0..rows {
      for x in 0..cols {
        if expanded_grid[y][x] == 2 && !visited[y][x] {
          if let Some(sides) =
            self.trace_and_count_perimeter(&expanded_grid, &mut visited, (x, y))
          {
            total_sides += sides;
          }
        }
      }
    }

    total_sides
  }
}

struct UnionFind {
  parent: Vec<usize>,
  rank: Vec<usize>,
}

impl UnionFind {
  fn new(size: usize) -> Self {
    UnionFind { parent: (0..size).collect(), rank: vec![0; size] }
  }

  /// Finds the root of element `x` with path compression.
  fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
      self.parent[x] = self.find(self.parent[x]); // Path compression.
    }
    self.parent[x]
  }

  /// Unites the sets containing `x` and `y`.
  fn union(&mut self, x: usize, y: usize) {
    let x_root = self.find(x);
    let y_root = self.find(y);

    if x_root == y_root {
      return;
    }

    // Union by rank using match statement
    match self.rank[x_root].cmp(&self.rank[y_root]) {
      Ordering::Less => {
        self.parent[x_root] = y_root;
      }
      Ordering::Greater => {
        self.parent[y_root] = x_root;
      }
      Ordering::Equal => {
        self.parent[y_root] = x_root;
        self.rank[x_root] += 1;
      }
    }
  }
}

pub type ProblemDefinition = Grid<Cell>;
pub type Consequent = HashMap<usize, Garden>;


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
  use std::collections::HashMap;

  use crate::{
    src_provider, Cell, Consequent, Garden, Point, ProblemDefinition,
    UnionFind, WrappedGrid,
  };


  pub fn extract() -> Result<ProblemDefinition, String> {
    src_provider()?
      .trim()
      .parse()
      .map_err(|_| "Error parsing grid".into())
  }

  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let grid = WrappedGrid(&data);

    let rows = data.height();
    let cols = data.width();
    let size = rows * cols;

    let mut uf = UnionFind::new(size);
    let mut label_grid = vec![' '; size];

    for (point, cell) in data.iter::<Point>() {
      let Cell::Plant(label) = cell;
      label_grid[point.to_index(cols)] = label;
    }

    // Union adjacent cells with the same label
    for (point, cell) in data.iter::<Point>() {
      let Cell::Plant(label) = cell;
      let idx = point.to_index(cols);

      for neighbor in grid.neighbors(&point) {
        let Cell::Plant(neighbor_label) = data[neighbor];

        if neighbor_label == label {
          let neighbor_idx = neighbor.to_index(cols);
          uf.union(idx, neighbor_idx);
        }
      }
    }

    let mut regions: HashMap<usize, Garden> = HashMap::new();

    for (point, Cell::Plant(label)) in data.iter::<Point>() {
      let root = uf.find(point.to_index(cols));

      let perimeter = grid.neighbors(&point).fold(4, |perimeter, neighbor| {
        let Cell::Plant(neighbor_label) = data[neighbor];
        if neighbor_label == label {
          perimeter - 1
        } else {
          perimeter
        }
      });

      #[cfg(not(feature = "part2"))]
      regions
        .entry(root)
        .and_modify(|garden| {
          garden.area += 1;
          garden.perimeter += perimeter;
        })
        .or_insert(Garden { label, area: 1, perimeter });
      #[cfg(feature = "part2")]
      regions
        .entry(root)
        .and_modify(|garden| {
          garden.area += 1;
          garden.points.push(point);
        })
        .or_insert(Garden { label, area: 1, perimeter, points: vec![point] });
    }

    #[cfg(feature = "part2")]
    regions.iter_mut().for_each(|(_, garden)| {
      garden.perimeter = garden.calculate_sides();
    });

    Ok(regions)
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(data) => {
        let total: usize = data
          .values()
          .map(|garden| garden.area * garden.perimeter)
          .sum();

        println!("Total cost {total}");

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
  #[test]
  #[cfg(all(feature = "sample", not(feature = "part2")))]
  fn test_transform() {
    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("Failed to transform data");

    println!("{:?}", result);

    let total: usize = result
      .values()
      .map(|garden| garden.area * garden.perimeter)
      .sum();

    assert_eq!(total, 1930);
  }

  #[test]
  #[cfg(all(feature = "sample", feature = "part2"))]
  #[mry::lock(src_provider)]
  fn test_transform_part2_mini() {
    mock_src_provider()
      .returns(Ok(include_str!("../sample.mini.txt").to_string()));
    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("Failed to transform data");

    println!("{:?}", result);

    let total: usize = result
      .values()
      .map(|garden| garden.area * garden.perimeter)
      .sum();

    assert_eq!(total, 80);
  }

  #[test]
  #[cfg(all(feature = "sample", feature = "part2"))]
  #[mry::lock(src_provider)]
  fn test_transform_part2_xo() {
    mock_src_provider()
      .returns(Ok(include_str!("../sample.xo.txt").to_string()));
    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("Failed to transform data");

    println!("{:?}", result);

    let total: usize = result
      .values()
      .map(|garden| garden.area * garden.perimeter)
      .sum();

    assert_eq!(total, 436);
  }

  #[test]
  #[cfg(all(feature = "sample", feature = "part2"))]
  #[mry::lock(src_provider)]
  fn test_transform_part2_ex() {
    mock_src_provider()
      .returns(Ok(include_str!("../sample.ex.txt").to_string()));
    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("Failed to transform data");

    println!("{:?}", result);

    let total: usize = result
      .values()
      .map(|garden| garden.area * garden.perimeter)
      .sum();

    assert_eq!(total, 236);
  }

  #[test]
  #[cfg(all(feature = "sample", feature = "part2"))]
  #[mry::lock(src_provider)]
  fn test_transform_part2_ab() {
    mock_src_provider()
      .returns(Ok(include_str!("../sample.ab.txt").to_string()));
    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("Failed to transform data");

    println!("{:?}", result);

    let total: usize = result
      .values()
      .map(|garden| garden.area * garden.perimeter)
      .sum();

    assert_eq!(total, 368);
  }

  #[test]
  #[cfg(all(feature = "sample", feature = "part2"))]
  #[mry::lock(src_provider)]
  fn test_transform_part2_l() {
    mock_src_provider()
      .returns(Ok(include_str!("../sample.l.txt").to_string()));
    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("Failed to transform data");

    println!("{:?}", result);

    let total: usize = result
      .values()
      .map(|garden| garden.area * garden.perimeter)
      .sum();

    assert_eq!(total, 160);
  }

  #[test]
  #[cfg(all(feature = "sample", feature = "part2"))]
  #[mry::lock(src_provider)]
  fn test_transform_part2_dimples() {
    mock_src_provider()
      .returns(Ok(include_str!("../sample.dimples.txt").to_string()));
    let data = extract().expect("Failed to extract data");
    let result = transform(data).expect("Failed to transform data");

    println!("{:?}", result);

    let total: usize = result
      .values()
      .map(|garden| garden.area * garden.perimeter)
      .sum();

    assert_eq!(total, 16 * 20 + 4 + 4 + 4 * 4 + 2 * 4);
  }

  // MARK load
}
