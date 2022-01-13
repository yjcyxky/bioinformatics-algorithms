use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect ["CATG", "GCAT"]
  println!(
    "Frequent Word: {:?}",
    dna_replication::find_frequent_word("ACGTTGCATGTCGCATGATGCATGAGAGCT", 4)
  );
}
