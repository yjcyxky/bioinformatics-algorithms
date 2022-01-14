use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect [2, 1, 0, 0, 0, 0, 2, 2, 1, 2, 1, 0, 0, 1, 1, 0]
  let results = dna_replication::compute_freq_array("ACGCGGCTCTGAAA", 2);
  println!("Patterns: {:?}", results);
}
