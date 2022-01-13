use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  let count = dna_replication::count_pattern("GCGCG", "GCG");
  // Expect 2
  println!("Count: {}", count);
}
