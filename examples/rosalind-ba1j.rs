use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  let results = dna_replication::find_freq_pattern_misrev("ACGTTGCATGTCGCATGATGCATGAGAGCT", 4, 1);
  println!("Patterns: {:?}", dna_replication::find_max_value(&results));
}
