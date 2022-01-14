use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect ["CCG", "TCG", "GCG", "AAG", "ATG", "AGG", "ACA", "ACC", "ACT", "ACG"]
  let results = dna_replication::neighbors("ACG", 1);
  println!("Patterns: {:?}", results);
}
