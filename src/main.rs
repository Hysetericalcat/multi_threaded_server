mod word_count;

fn main() {
  let sentence = String::from("Hello rust Hello rust");
  word_count::word_count(&sentence);
}
