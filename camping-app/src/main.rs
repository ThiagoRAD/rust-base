fn sentence_builder(person_info: (&str, u64, &str)) {
  println!("{} is {}, and her last initial is {}.", person_info.0, person_info.1, person_info.2)
}

fn main() {
  let person_info = ("Eve", 38, "P");
  println!("{:?}", person_info);
  sentence_builder(person_info);
}
