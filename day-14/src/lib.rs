mod object_zoo;
mod parse;
#[cfg(feature = "part2")]
mod part2;

use crate::object_zoo::prelude::*;

#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

pub type Consequent = [u16; 4];


#[cfg(test)]
#[mry::mry]
fn get_dims() -> Point<usize> {
  Point { x: X_DIM, y: Y_DIM }
}
#[cfg(not(test))]
fn get_dims() -> Point<usize> {
  Point { x: X_DIM, y: Y_DIM }
}

#[cfg(not(feature = "part2"))]
fn get_quadrant(point: Point<u8>, dims: &Point<usize>) -> Option<usize> {
  let mid_x = dims.x / 2;
  let mid_y = dims.y / 2;

  // Exclude robots on mid_x or mid_y
  if point.x == mid_x as u8 || point.y == mid_y as u8 {
    None
  } else if point.x < mid_x as u8 {
    if point.y < mid_y as u8 {
      Some(0) // Top-left
    } else {
      Some(2) // Bottom-left
    }
  } else if point.y < mid_y as u8 {
    Some(1) // Top-right
  } else {
    Some(3) // Bottom-right
  }
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
  use crate::object_zoo::ProblemDefinition;
  #[cfg(not(feature = "part2"))]
  use crate::object_zoo::{get_duration, prelude::Point};
  use crate::parse::parse_problem_definition;
  #[cfg(feature = "part2")]
  use crate::part2::transform_part2_bevy;

  #[cfg(not(feature = "part2"))]
  use crate::get_quadrant;
  use crate::{get_dims, src_provider, Consequent};

  pub fn extract() -> Result<ProblemDefinition, String> {
    match src_provider() {
      Ok(data) => match parse_problem_definition(&data) {
        Ok((_, problem_def)) => Ok(problem_def),
        Err(e) => Err(format!("Failed to parse data: {:?}", e)),
      },
      Err(e) => Err(e),
    }
  }

  #[allow(unused_mut)]
  pub fn transform(mut data: ProblemDefinition) -> Result<Consequent, String> {
    let dims = get_dims();
    #[cfg(not(feature = "part2"))]
    {
      let duration = get_duration();
      let mut quadrant_safety = [0; 4];

      for robot in data.robots.iter_mut() {
        let x = (robot.position.x as isize
          + duration as isize * robot.velocity.x as isize)
          .rem_euclid(dims.x as isize) as u8;
        let y = (robot.position.y as isize
          + duration as isize * robot.velocity.y as isize)
          .rem_euclid(dims.y as isize) as u8;

        if let Some(index) = get_quadrant(Point::<u8> { x, y }, &dims) {
          quadrant_safety[index] += 1;
        }
      }

      return Ok(quadrant_safety);
    }

    #[cfg(feature = "part2")]
    {
      transform_part2_bevy(data, dims);

      Ok([0; 4])
    }
  }


  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    match result {
      Ok(quadrant_safety) => {
        let quadrant_safety =
          quadrant_safety.iter().fold(1, |acc, &x| acc * x as u32);
        println!("Quadrant safety: {}", quadrant_safety);

        Ok(())
      }
      Err(e) => Err(format!("Failed to load result: {}", e)),
    }
  }
}


#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::{prelude::*, *};
  #[allow(unused_imports)]
  use object_zoo::*;
  #[allow(unused_imports)]
  use parse::*;

  #[test]
  fn test_parse_problem_definition() {
    let input = "p=0,4 v=3,-3\np=6,3 v=-1,-3";
    let (_, problem_definition) = parse_problem_definition(input)
      .expect("Failed to parse problem definition");

    // Validate the number of robots parsed
    assert_eq!(problem_definition.robots.len(), 2);

    // Validate the first robot's data
    assert_eq!(
      problem_definition.robots[0],
      Robot { position: Point { x: 0, y: 4 }, velocity: Point { x: 3, y: -3 } }
    );

    // Validate the second robot's data
    assert_eq!(
      problem_definition.robots[1],
      Robot {
        position: Point { x: 6, y: 3 },
        velocity: Point { x: -1, y: -3 },
      }
    );
  }


  // MARK extract
  #[test]
  #[mry::lock(src_provider)]
  fn test_expect() {
    // Exact input provided
    let input = "p=0,4 v=3,-3\np=6,3 v=-1,-3\np=10,3 v=-1,2\np=2,0 \
                 v=2,-1\np=0,0 v=1,3\np=3,0 v=-2,-2\np=7,6 v=-1,-3\np=3,0 \
                 v=-1,-2\np=9,3 v=2,3\np=7,3 v=-1,2\np=2,4 v=2,-3\np=9,5 \
                 v=-3,-3"
      .to_string();

    // Parse the input
    mock_src_provider().returns(Ok(input));
    let result = extract().expect("Failed to extract data");

    // Verify the number of robots
    assert_eq!(result.robots.len(), 12);

    // Verify some specific robots
    assert_eq!(result.robots[0].position, Point { x: 0, y: 4 });
    assert_eq!(result.robots[0].velocity, Point { x: 3, y: -3 });

    assert_eq!(result.robots[1].position, Point { x: 6, y: 3 });
    assert_eq!(result.robots[1].velocity, Point { x: -1, y: -3 });

    assert_eq!(result.robots[2].position, Point { x: 10, y: 3 });
    assert_eq!(result.robots[2].velocity, Point { x: -1, y: 2 });

    // Add more specific checks as needed
  }


  // MARK transform
  #[cfg(all(feature = "sample", not(feature = "part2")))]
  #[test]
  #[mry::lock(get_dims)]
  fn test_transform() {
    mock_get_dims().returns(Point::<usize> { x: 11, y: 7 });

    dbg!(get_dims(), get_duration());

    let data = extract().expect("failed to extract data");
    let result = transform(data).expect("failed to transform data");

    dbg!(result);

    assert_eq!(result, [1_u16, 3, 4, 1])
  }

  // MARK load
}
