use array_tool::vec::Intersect;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);
  
  let lines: Vec<String> = reader.lines()
    .map(|l| l.expect("Couldn't read a line"))
    .collect();
  let mut group: Vec<char> = vec![]; 
  let mut sum = 0;
  let mut count = 0;

  // Cal groups
  for line in &lines {
    let trimmed_line = line.trim_end();
    if trimmed_line.is_empty() {
      sum += group.len();
      println!("{:?} {} {}", group, group.len(), sum);
      group = vec![]; 
      count = 0;
    } else {
      let mut char_vec: Vec<char> = trimmed_line.chars().collect();
      if count == 0 {
        group.append(&mut char_vec);
      } else {
        group = group.intersect(char_vec);
      }
      count += 1;
      // println!("{:?}", group);
    }
  }

  println!("Found {} YES answers!", sum);
  Ok(())
}