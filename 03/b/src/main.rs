use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);
  
  let lines: Vec<String> = reader.lines()
    .map(|l| l.expect("Couldn't read a line"))
    .collect();
  let heigth = lines.len();

  let slopes = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

  // repeat the pattern
  let mut new_lines: Vec<String> = [].to_vec();
  for line in &lines {
    new_lines.push(line.repeat(206));
  }

  let tree = '#';
  let mut product: u64 = 1;
  for slope in &slopes {
    let mut count = 0;
    let down = slope.1;
    let step_to_add = slope.0;
    let mut step_index = 0;

    for x in (down..heigth).step_by(down) {
      let line = &new_lines[x];
      step_index += step_to_add;
      let line_char = line.chars().nth(step_index as usize).unwrap();
      if line_char == tree {
        count += 1;
      }
    }
    product *= count;
    println!("Found {} trees!", count);
  }
  println!("Product {} !", product);  

  Ok(())
}