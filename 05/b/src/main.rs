use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);
  
  let lines: Vec<String> = reader.lines()
    .map(|l| l.expect("Couldn't read a line"))
    .collect();

  let mut ids: Vec<f64> = vec![];
  let mut places: HashMap<String, Vec<f64>> = HashMap::new();
  for line in lines {
    let mut min_row: f64 = 0.0;
    let mut max_row: f64 = 127.0;
    let mut min_column: f64 = 0.0;
    let mut max_column: f64 = 7.0;
    // println!("line {}", line);
    // println!("init {} {} {} {}", min_row, max_row, min_column, max_column);
    for letter in line.chars() {
      let row_value = (max_row - min_row) / 2.0 + min_row;
      let column_value = (max_column - min_column) / 2.0 + min_column;
      match letter {
        'F' => {
          max_row = row_value.floor();
        },
        'B' => {
          min_row = row_value.ceil();
        },
        'L' => {
          max_column = column_value.floor();
        },
        'R' => {
          min_column = column_value.ceil();
        },
        _ => { println!("Invalid letter") }
      }
      // println!("{} {} {} {} {}", min_row, max_row, min_column, max_column, letter);
    }
    ids.push((min_row * 8.0) + min_column);
    let key = min_row.to_string();
    places.entry(key).or_insert(vec![]).push(min_column);
  }
  println!("Found {} boarding pass", ids.len());

  ids.sort_by(|a, b| a.partial_cmp(b).unwrap());

  match ids.last() {
    Some(max) => println!( "Max Seat id: {}", max ),
    None      => println!( "Vector is empty" ),
  }

  for (row, columns) in places {
    if columns.len() == 7 {
      for x in 0..7 {
        if !columns.contains(&(x as f64)) {
          let seat_id: u32 = row.parse::<u32>().unwrap() * 8 + x;
          println!("Missing seat ID {}, row {} column {}", seat_id, row, x);
        }
      }
    }
  }
  Ok(())
}