#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

struct ProblemDefinition {}
type Consequent = String;


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
  todo!()
}

fn transform(_data: ProblemDefinition) -> Result<Consequent, String> {
  todo!()
}

fn load(_result: Result<Consequent, String>) -> Result<(), String> {
  todo!()
}

fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}


#[cfg(test)]
mod tests {
  // use super::*;

  // MARK extract
  // #[mry::lock(src_provider)] // Lock the function for mocking.

  // MARK transform

  // MARK load
}
