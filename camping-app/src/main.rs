fn main() {
  let mut packing_list = Vec::new();
  packing_list.push("Sunglasses");
  packing_list.push("Sunscreen");
  packing_list.push("Hat");
  println!("{:?}", packing_list);

  let second_packing_list = vec!["Sunglasses", "Sunscreen", "Hat"];
  println!("{:?}", second_packing_list);
}
