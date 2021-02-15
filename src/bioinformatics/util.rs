extern crate csv;

use csv::Writer;
use std::error::Error;

pub fn write_csv(results: Vec<Vec<i32>>, output: &str) -> Result<(), Box<dyn Error>> {
  let mut wtr = Writer::from_path(output)?;

  wtr.write_record(&["result", "pos"])?;

  for (idx, _) in results[0].iter().enumerate() {
    wtr.serialize((results[0][idx], results[1][idx]))?;
  }

  wtr.flush()?;
  Ok(())
}
