use day_14::prelude::*;

mod object_zoo;
mod parse;


fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}
