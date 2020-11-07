use bio::io::fasta;
use std::path::Path;
use std::str;

pub fn read_fasta(filepath: &str) -> String {
  let path = Path::new(filepath);
  let reader = fasta::Reader::from_file(path).unwrap();
  let mut seq_string: String = String::from("");

  for record in reader.records() {
    let record = record.unwrap();
    let seq = record.seq().to_vec();
    seq_string = str::from_utf8(&seq).unwrap().to_string();
  }

  return seq_string;
}
