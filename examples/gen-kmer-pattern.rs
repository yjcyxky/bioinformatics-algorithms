use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  let results = dna_replication::gen_kmer_patterns(3);
  println!("Patterns: {:?}", results);
  println!("Num of Patterns: {:?}", results.len());
}
