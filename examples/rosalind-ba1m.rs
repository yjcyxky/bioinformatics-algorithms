use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect AGTC
  // Rule: AGTC --> 4 * (4 * (4 * 0 + 2) + 3) + 2
  let results = dna_replication::number2kmer_pattern(45, 4);
  println!("Patterns: {:?}", results);
}
