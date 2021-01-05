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
    let first_index: u32 = range_string[0].parse().expect("Wanted a number");
    let second_index: u32 = range_string[1].parse().expect("Wanted a number");
    let first_idx: usize = (first_index - 1) as usize;
    let second_idx: usize = (second_index - 1) as usize;
    let password = splitted[2];
    let test_char = splitted[1].replace(":", "").chars().nth(0).unwrap();
    let first_char = password.chars().nth(first_idx).unwrap();
    let second_char = password.chars().nth(second_idx).unwrap();
    let test_one = if first_char == test_char { 1 } else { 0 };
    let test_two = if second_char == test_char { 1 } else { 0 };
    let sum = test_one + test_two;
    if sum == 1 {
      count += 1;
    }
    // println!("Password {}", password);
    // println!("{} {} {} {}", password , first_idx, first_char, alphabet[first_idx]);
    // println!("{} {} {} {}", password , second_idx, second_char, alphabet[second_idx]);
    // println!("Password {}", password);
    // println!("Found chars {}", found_chars);
  }

  println!("Found {} valid passwords!", count);
  Ok(())
}