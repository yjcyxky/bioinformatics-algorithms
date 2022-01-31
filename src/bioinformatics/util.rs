extern crate csv;

use csv::Writer;
use std::error::Error;
use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

pub fn write_csv(results: Vec<Vec<i32>>, output: &str) -> Result<(), Box<dyn Error>> {
  let mut wtr = Writer::from_path(output)?;

  wtr.write_record(&["result", "pos"])?;

  for (idx, _) in results[0].iter().enumerate() {
    wtr.serialize((results[0][idx], results[1][idx]))?;
  }

  wtr.flush()?;
  Ok(())
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
