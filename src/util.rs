extern crate csv;

use std::error::Error;
use csv::Writer;

pub fn write_csv(results: Vec<Vec<i32>>) -> Result<(), Box<dyn Error>> {
  let mut wtr = Writer::from_path("foo.csv")?;

  wtr.write_record(&["result", "pos"])?;

  for (idx, _) in results[0].iter().enumerate() {
    wtr.serialize((results[0][idx], results[1][idx]))?;
  }

  wtr.flush()?;
  Ok(())
}
