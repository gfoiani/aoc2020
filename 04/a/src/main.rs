use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);
  
  let keys: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
  let keys_len = keys.len();
  println!("{:?}", keys);
  let lines: Vec<String> = reader.lines()
    .map(|l| l.expect("Couldn't read a line"))
    .collect();
  let mut passports: Vec<HashMap<&str, &str>> = vec![]; 
  let mut count = 0;
  let mut element: HashMap<&str, &str> = HashMap::new();

  // Generate passports
  for line in &lines {
    let trimmed_line = line.trim_end();
    if trimmed_line.is_empty() {
      passports.push(element);
      element = HashMap::new();
    } else {
      let fields: Vec<&str> = trimmed_line.split_whitespace().collect();
      for field in fields {
        let data: Vec<&str> = field.split(":").collect();
        element.insert(data[0], data[1]);
      }
    }
  }
  println!("Found {} passports!", passports.len());
  for passport in passports {
    let passport_keys_len = passport.keys().len();
    let condition = (passport_keys_len == keys_len) || (!passport.contains_key("cid") && (passport_keys_len >= (keys_len - 1)));
    if condition {
      count += 1;
    }
  }

  println!("Found {} valid passports!", count);
  Ok(())
}