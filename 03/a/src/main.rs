use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);
  
  let lines: Vec<String> = reader.lines()
    .map(|l| l.expect("Couldn't read a line"))
    .collect();
  let _heigth = lines.len();
  let width = lines[0].len();

  let mut count = 0;
  let tree = '#';
  let step_to_add = 3;
  let mut step_index = 0;
  
  // repeat the pattern
  let mut new_lines: Vec<String> = [].to_vec();
  for line in &lines {
    new_lines.push(line.repeat(width+1));
  }
  // println!("{:?}", new_lines[0]);
  // println!("heigth {}", new_lines.len());
  // println!("width {}", new_lines[0].len());

  for line in new_lines {
    let line_char = line.chars().nth(step_index as usize).unwrap();
    if line_char == tree {
      count += 1;
    }
    step_index += step_to_add;
  }

  println!("Found {} trees!", count);
  Ok(())
}