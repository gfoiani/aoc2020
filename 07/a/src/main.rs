use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);
  
  let lines: Vec<String> = reader.lines()
    .map(|l| l.expect("Couldn't read a line"))
    .collect();
  let mut bags: HashMap<&str,HashMap<String, u32>> = HashMap::new();

  // Get bags combinations
  for line in &lines {
    let trimmed_line = line.trim_end();
    let first_split: Vec<&str> = trimmed_line.split(" bags contain ").collect();
    let mut bags_to_add: HashMap<String, u32> = HashMap::new();
    if first_split[1].chars().any(char::is_numeric) {
      let second_split: Vec<&str> = first_split[1].split(", ").collect();
      for bag in second_split {
        let third_split: Vec<&str> = bag.split(" ").collect();
        let color: String = third_split[1].to_string() + " " + third_split[2].replace(".", "").as_str();
        let count: u32 = third_split[0].to_string().parse().unwrap();
        // println!("{} {}", count, color);
        bags_to_add.insert(color, count);
      }
    } else {
      println!("No bags contained in {}", first_split[0]);
    }

    // println!("{:?}", bags_to_add);
    bags.insert(first_split[0], bags_to_add);
  }

  println!("{} bags rules", bags.len());
  // println!("{:?}", bags);
  let mut bag_colors: Vec<String> = vec![String::from("shiny gold")];
  let mut found_bags: Vec<String> = vec![];
  let mut all_bags: Vec<String> = vec![];
  let mut loop_count = 0;
  let mut total_count = 0;
  loop {
    println!("loop_count {}", loop_count);
    for (bag, contained) in &bags {
      // do not check the same bag multple times
      // println!("{} all_bags {:?}", all_bags.len(), all_bags);
      // println!("bag {}", bag);
      if all_bags.contains(&bag.to_string()) {
        continue;
      }
      for inner_bag in &bag_colors {
        if contained.contains_key(inner_bag) {
          found_bags.push(bag.to_string());
        }
      }
    }
    found_bags.sort_unstable();
    found_bags.dedup();    
    println!("{} found_bags {:?}", found_bags.len(), found_bags);
    total_count += found_bags.len();
    if found_bags.len() == 0 {
      break;
    }
    all_bags.append(& mut found_bags.clone());
    bag_colors = found_bags;
    found_bags = vec![];
    loop_count += 1;
  }
  println!("{} total_count", total_count);

  Ok(())
}