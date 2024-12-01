fn main() {
  let summer = true;
  let winter = true;
  println!("Summer is the best season: {}", summer);
  println!("Winter is also good: {}", winter);

  if summer {
    println!("go biking!");
  } else if winter {
    println!("go skiing!");
  } else {
    println!("do something else!");
  }
}
