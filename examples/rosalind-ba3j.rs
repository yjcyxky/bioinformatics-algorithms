use bioinformatics_algorithms::bioinformatics::dna_replication;
use bioinformatics_algorithms::bioinformatics::util;

#[warn(unused_must_use)]
fn main() {
  // Expect:
  // ```
  // GTGGTCGTGAGATGTTGA
  // ```
  // let patterns = vec![
  //   "GAGA|TTGA",
  //   "TCGT|GATG",
  //   "CGTG|ATGT",
  //   "TGGT|TGAG",
  //   "GTGA|TGTT",
  //   "GTGG|GTGA",
  //   "TGAG|GTTG",
  //   "GGTC|GAGA",
  //   "GTCG|AGAT"
  // ];
  // let k = 4;
  // let d = 2;

  let mut content: Vec<String> = vec![];
  match util::read_lines("./examples/data/rosalind-ba3j.txt") {
    Ok(lines) => {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
        match line {
          Ok(pattern) => content.push(pattern),
          Err(err) => {
            println!("Error: {:?}", err.to_string())
          }
        }
      }

      println!("File Content: {:?}", content);
      let header: Vec<usize> = content[0]
        .split_whitespace()
        .map(|num| num.to_string().parse::<usize>().unwrap())
        .collect();
      let k = header[0];
      let d = header[1];

      let patterns = content[1..]
        .into_iter()
        .map(|pattern| &pattern[..])
        .collect();
      let results = dna_replication::read_pairs(&patterns, k);
      println!("Read Pairs: {:?}", results.output_paired_text(k, d));
    }
    Err(err) => {
      println!("Error: {:?}", err.to_string())
    }
  }
}
