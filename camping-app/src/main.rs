use std::fs::OpenOptions;
use std::io::Write;

fn main() {
  let mut file = OpenOptions::new()
    .append(true)
    .read(true)
    .write(true)
    .create(true)
    .open("./my_file.txt")
    .expect("Something went wrong opening the file");
  let text = "We're making it happen!";
  file.write_all(text.as_bytes()).expect("Something went wrong writing to the file");
}





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
