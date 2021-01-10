use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);
  
  let lines: Vec<String> = reader.lines()
    .map(|l| l.expect("Couldn't read a line"))
    .collect();

  let mut boards: Vec<HashMap<&str, f64>> = vec![];
  let mut board: HashMap<&str, f64> = HashMap::new();
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
    board.insert("row", min_row);
    board.insert("column", min_column);
    board.insert("seat_id", (min_row * 8.0) + min_column);
    // println!("row {}", min_row);
    // println!("column {}", min_column);
    // println!("seat_id {}", (min_row * 8.0) + min_column);
    boards.push(board);
    board = HashMap::new();
  }
  println!("Found {} boarding pass", boards.len());
  let mut ids: Vec<f64> = boards.iter().map(|x| x["seat_id"]).collect();
  ids.sort_by(|a, b| a.partial_cmp(b).unwrap());
  match ids.last() {
    Some(max) => println!( "Max Seat id: {}", max ),
    None      => println!( "Vector is empty" ),
  }
  Ok(())
}