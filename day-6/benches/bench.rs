use day_6::prelude::*;


#[divan::bench]
fn extract_benchmark() {
  extract().unwrap();
}

#[divan::bench]
fn transform_benchmark() {
  let data = divan::black_box(extract().expect("Failed to extract data"));

  transform(data).unwrap();
}

#[divan::bench]
fn work_benchmark() {
  let data = extract().expect("Failed to extract data");

  let result = transform(data);
  load(result).unwrap();
}

#[divan::bench]
fn main_bench() {
  let data = extract().expect("Failed to extract data");
  let result = transform(data);
  load(result).unwrap();
}

fn main() {
  divan::main();
}
