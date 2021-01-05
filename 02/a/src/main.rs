use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);
  
  let lines: Vec<String> = reader.lines()
    .map(|l| l.unwrap().parse().expect("Couldn't read a line"))
    .collect();

  let mut count = 0;

  for line in lines {
    let splitted: Vec<&str> = line.split_whitespace().collect();
    let range_string: Vec<&str> = splitted[0].split("-").collect();
    let min: u32 = range_string[0].parse().expect("Wanted a number");
    let max: u32 = range_string[1].parse().expect("Wanted a number");
    let range = min..=max;
    let test_char = splitted[1].replace(":", "");
    let password = splitted[2];
    let found_chars: u32 = password.matches(&test_char).count() as u32;
    // println!("Range min: {:?} max: {:?}", min, max);
    // println!("Test char {}", test_char);
    // println!("Password {}", password);
    // println!("Found chars {}", found_chars);
    if range.contains(&found_chars) {
      count += 1;
    }
  }

  println!("Found {} valid passwords!", count);
  Ok(())
}