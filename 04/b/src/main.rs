use regex::Regex;
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
  let colors: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
  for passport in passports {
    let passport_keys_len = passport.keys().len();
    let condition = (passport_keys_len == keys_len) || (!passport.contains_key("cid") && (passport_keys_len >= (keys_len - 1)));
    if condition {
      let mut passport_ok = true;
      for (key, value) in passport {
        // println!("{} {}", key, value);
        match key {
          "byr" => {
            if value.len() != 4 || !(1920..=2002).contains(&value.parse().unwrap()) {
              passport_ok = false;
              break;
            }
          }
          "iyr" => {
            if value.len() != 4 || !(2010..=2020).contains(&value.parse().unwrap()) {
              passport_ok = false;
              break;
            }
          },
          "eyr" => {
            if value.len() != 4 || !(2020..=2030).contains(&value.parse().unwrap()) {
              passport_ok = false;
              break;
            }
          },
          "hgt" => {
            let value_len = value.len();
            let measure_unit = &value[value_len-2..];
            let measure = value.to_string().replace(measure_unit, "");
            // println!("{} {}", measure, measure_unit);         
            let range;
            match measure_unit {
              "in" => { range = 59..=76 },
              "cm" => { range = 150..=193 } ,
              _ => {
                println!("Invalid Measure unit!");
                passport_ok = false;
                break;
              },
            }
            if !range.contains(&measure.parse().unwrap()) {
              passport_ok = false;
              break;
            }
          },
          "hcl" => {
            let re = Regex::new(r"^#[0-9,a-f]{6}$").unwrap();
            if !re.is_match(value) {
              passport_ok = false;
              break;
            }
          },
          "ecl" => {
            if !colors.contains(&value) {
              passport_ok = false;
              break;
            }
          },
          "pid" => {
            let re = Regex::new(r"^\d{9}$").unwrap();
            if !re.is_match(value) {
              passport_ok = false;
              break;
            }
          },
          "cid" => {},
          _ => println!("Invalid Key!"),
        }
        
      }
      if passport_ok {
        count += 1;
      }
    }
  }

  println!("Found {} valid passports!", count);
  Ok(())
}