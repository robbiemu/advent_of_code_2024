use day_13::prelude::*;


fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}
