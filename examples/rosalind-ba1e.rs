use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect ["CGACA", "GAAGA", "AATGT"]
  println!("Clumps: {:?}", dna_replication::find_clump("CGGACTCGACAGATGTGAAGAAATGTGAAGACTGAGTGAAGAGAAGAGGAAACACGACACGACATTGCGACATAATGTACGAATGTAATGTGCCTATGGC", 75, 5, 4));
}
