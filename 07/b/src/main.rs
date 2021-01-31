use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// thanks to andreamazz
fn count_in(color: String, relationships: &HashMap<String,HashMap<String, u32>>) -> u32 {
  let relationship: &HashMap<String, u32> = relationships.get(&color).unwrap();
  // println!("color {}", color);
  // println!("{:?}", relationship);
  let keys = relationship.keys();
  if keys.len() == 0 {
    return 1;
  }
  return keys.map(|b| count_in(b.to_string(), relationships) * relationship[b]).sum::<u32>() + 1;
}

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);
  
  let lines: Vec<String> = reader.lines()
    .map(|l| l.expect("Couldn't read a line"))
    .collect();
  let mut bags: HashMap<String,HashMap<String, u32>> = HashMap::new();

  // Get bags combinations
  for line in &lines {
    let trimmed_line = line.trim_end();
    let first_split: Vec<&str> = trimmed_line.split(" bags contain ").collect();
    let mut bags_to_add: HashMap<String, u32> = HashMap::new();
    if first_split[1].chars().any(char::is_numeric) {
      let second_split: Vec<&str> = first_split[1].split(", ").collect();
      for bag in second_split {
        let third_split: Vec<&str> = bag.split(" ").collect();
        let color: String = format!("{} {}", third_split[1].to_string(), third_split[2].replace(".", ""));
        let count: u32 = third_split[0].to_string().parse().unwrap();
        // println!("{} {}", count, color);
        bags_to_add.insert(color, count);
      }
    } else {
      println!("No bags contained in {}", first_split[0]);
    }

    bags.insert(first_split[0].to_string(), bags_to_add);
  }

  // println!("{} bags rules", bags.len());
  // println!("{:?}", bags);
  let res = count_in("shiny gold".to_string(), &bags) - 1;
  println!("res {}", res);

  Ok(())
}