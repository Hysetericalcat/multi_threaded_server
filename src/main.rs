mod minigrep;
use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let content = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
  let mut config = minigrep::Config{query:&args[2],filename: &args[1],content:&content};
  let results = config.search();
  dbg!(results);
}
