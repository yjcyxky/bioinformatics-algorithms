use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect [6, 7, 26, 27, 78]
  let count = dna_replication::count_pattern_with_mismatches("CGCCCGAATCCAGAACGCATTCCCATATTTCGGGACCACTGGCCTCCACGGTACGGACGTCAATCAAATGCCTAGCGGCTTGTGGTTTCTCCTACGCTCC", "ATTCTGGA", 3);
  println!("Count: {:?}", count);
}
