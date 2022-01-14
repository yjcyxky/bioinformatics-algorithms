use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect 11
  let results = dna_replication::pattern2number("TAGACGGAGGCTAACACTTC");
  println!("Patterns: {:?}", results);
}
