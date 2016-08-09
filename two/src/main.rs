use std::io;

fn main() {
  println!("Hello, world!");

  let x: Vec<u32> = vec![1,2,3];
  let y: Vec<u32> = {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
  };

  let (a,b) = (1,2);
  println!("{} {}", a, b);
}
