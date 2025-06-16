struct Hiker {
  name: String, 
  miles_hiked: u32,
}

fn main() {
  let jennifer = Hiker {
    name: String::from("Jennifer"),
    miles_hiked: 49,
  };
  println!("{} has hiked {} miles", jennifer.name, jennifer.miles_hiked);
  let bob = Hiker {
    name: String::from("Bob"),
    miles_hiked: 15,
  };
  println!("{} has hiked {} miles", bob.name, bob.miles_hiked);
}
