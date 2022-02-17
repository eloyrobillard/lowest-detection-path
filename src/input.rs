use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Input {
  pub length: f64,
  pub num_detectors: i32,
  pub detectors: Vec<(f64, f64)>,
}

pub fn format_input(input: String) -> Input {
  let mut result = Input {
      length: 0.0,
      num_detectors: 0,
      detectors: Vec::new()
  };

  let mut iter = input.split(|c: char| c.is_ascii_whitespace());

  result.length = iter.next().unwrap().parse().unwrap();
  result.num_detectors = iter.next().unwrap().parse().unwrap();

  while let Some(x_str) = iter.next() {
      let x: f64 = x_str.parse().unwrap();
      let y: f64 = iter.next().unwrap().parse().unwrap();
      result.detectors.push((x, y));
  }

  result
}

pub fn get_input(path: &str) -> String {
  // LINK https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
  let path = Path::new(path);
  let display = path.display();

  let mut file = match File::open(&path) {
      Err(why) => panic!("couldn't open {}: {}\n", display, why),
      Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
      Err(why) => panic!("couldn't read {}: {}\n", display, why),
      Ok(_) => {},
  }

  s
}