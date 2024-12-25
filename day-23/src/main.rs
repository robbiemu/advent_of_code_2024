use day_23::prelude::*;


fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}
