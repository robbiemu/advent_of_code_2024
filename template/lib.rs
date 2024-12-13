#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

pub struct ProblemDefinition {}
pub type Consequent = String;


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
  use crate::{src_provider, Consequent, ProblemDefinition};

  pub fn extract() -> Result<ProblemDefinition, String> {
    todo!()
  }

  pub fn transform(_data: ProblemDefinition) -> Result<Consequent, String> {
    todo!()
  }

  pub fn load(_result: Result<Consequent, String>) -> Result<(), String> {
    todo!()
  }
}


#[cfg(test)]
mod tests {
  //   #[allow(unused_imports)]
  //  use super::{prelude::*, *};

  // MARK extract
  // #[mry::lock(src_provider)] // Lock the function for mocking.

  // MARK transform

  // MARK load
}
