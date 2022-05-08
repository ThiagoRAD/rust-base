fn main() {
  struct Hiker {
    name: String, 
    miles_hiked: u32,
  }

  let billy = Hiker {
    name: "Billy".to_string(),
    miles_hiked: 32,
  };

  let Hiker { name, miles_hiked } = billy;

  println!("{} has hiked {} miles", name, miles_hiked);
}
// let destination = "Long Lake";

// match destination {
//   "Long Lake" => println!("We're heading to Long Lake!"),
//   "Mammoth Lakes" => println!("We're heading to Mammoth!"),
//   "Bowman Lake" => println!("We're heading to Bowman Lake!"),
//   _ => println!("Let's stay home!"),
// }


// use std::fs::OpenOptions;
// use std::io::Write;

// fn main() {
//   let mut file = OpenOptions::new()
//     .append(true)
//     .read(true)
//     .write(true)
//     .create(true)
//     .open("./my_file.txt")
//     .expect("Something went wrong opening the file");
//   let text = "We're making it happen!";
//   file.write_all(text.as_bytes()).expect("Something went wrong writing to the file");
// }





// fn main() {
//   let text = fs::read_to_string("./my_file.txt").expect("Something went wrong reading the file");
//   println!("What is in this file:\n{}", text);
// }


// struct Hiker {
//   name: String, 
//   miles_hiked: u32,
// }

// fn main() {
//   let jennifer = Hiker {
//     name: String::from("Jennifer"),
//     miles_hiked: 49,
//   };
//   println!("{} has hiked {} miles", jennifer.name, jennifer.miles_hiked);
//   let bob = Hiker {
//     name: String::from("Bob"),
//     miles_hiked: 15,
//   };
//   println!("{} has hiked {} miles", bob.name, bob.miles_hiked);
// }
