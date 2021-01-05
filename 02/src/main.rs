use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);
  
  let mut lines: Vec<u32> = reader.lines()
    .map(|l| l.unwrap().parse().expect("Couldn't read a line"))
    .collect();

  lines.sort();
  lines.dedup();

  let length = lines.len();
  println!("length {}", length);
  let mut found = false;
  let mut product = 0;
  for x in 0..length {
    for y in 0..length {
      for z in 0..length {
        let sum = lines[x] + lines[y] + lines[z];
        if sum == 2020 {
          found = true;
          product = lines[x] * lines[y] * lines[z];
          println!("X {} {}", x, lines[x]);
          println!("Y {} {}", y, lines[y]);
          println!("Z {} {}", z, lines[z]);
        }
        if sum > 2020 {
          break;
        }
      }
      if found == true {
        break;
      }
    }
    if found == true {
      break;
    }
  }

  println!("Found product {}!", product);
  Ok(())
}