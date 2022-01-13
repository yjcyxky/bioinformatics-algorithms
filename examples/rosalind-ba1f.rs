use bioinformatics_algorithms::bioinformatics::{dna_replication, fasta, util};

#[warn(unused_must_use)]
fn main() {
  let results = dna_replication::min_skew("CCTATCGGTGGATTAGCATGTCCCTGTACGTTTCGCCGCGAACTAGTTCACACGGCTTGATGGCAAATGGTTTTTCCGGCGACCGTAATCGTCCACCGAG", 0);
  println!("Min Skew: {:?}", results);
}
