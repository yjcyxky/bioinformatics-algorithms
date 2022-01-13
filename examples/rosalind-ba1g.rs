use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect 3
  println!(
    "Hamming Distance: {:?}",
    dna_replication::hamming_distance("GGGCCGTTGGT", "GGACCGTTGAC")
  );
}
