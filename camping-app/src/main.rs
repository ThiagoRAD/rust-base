#[derive(Debug)]
enum Steepness {
  Easy, 
  Moderate, 
  Difficult,
}

fn main() {
  let calm_trail = Steepness::Easy;
  println!("Steepness is {:?}", calm_trail);
  let fun_trail = Steepness::Moderate;
  println!("Steepness is {:?}", fun_trail);
  let pricly_peak_trail = Steepness::Difficult;
  println!("Steepness is {:?}", pricly_peak_trail);
}
