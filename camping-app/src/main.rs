fn calculate_distance(days: u64, distance: u64) -> Result<u64, String> {
  if days == 0 {
    return Err(String::from("Days cannot be zero"));
  }
  let total_miles = days * distance;
  return Ok(total_miles);
}

fn main() {
  let result = calculate_distance(5, 10);
  match result {
      Ok(miles) => println!("{} miles", miles),
      Err(e) => println!("Error: {}", e),
  }
}
