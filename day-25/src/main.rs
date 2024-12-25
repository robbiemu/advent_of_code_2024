use day_25::prelude::*;


fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}
