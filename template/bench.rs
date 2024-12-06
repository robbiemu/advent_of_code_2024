use gag::Gag;

use __DAY__::prelude::*;


#[cfg(feature = "full_bench")]
#[divan::bench]
fn extract_benchmark() {
  extract().unwrap();
}

#[cfg(feature = "full_bench")]
#[divan::bench]
fn transform_benchmark() {
  let data = divan::black_box(extract().expect("Failed to extract data"));

  transform(data).unwrap();
}

#[cfg(feature = "full_bench")]
#[divan::bench]
fn work_benchmark() {
  let data = extract().expect("Failed to extract data");
  transform(data).unwrap();
}

#[divan::bench]
fn main_bench() {
  let data = extract().expect("Failed to extract data");
  let result = transform(data);

  let gag = divan::black_box(Gag::stdout().unwrap());

  load(result).unwrap();

  drop(gag);
}

fn main() {
  divan::main();
}
