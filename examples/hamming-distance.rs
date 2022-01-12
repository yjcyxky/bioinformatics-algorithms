use algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  println!("Hamming Distance: {:?}", dna_replication::hamming_distance("GGGCCGTTGGT", "GGACCGTTGAC"));
}
