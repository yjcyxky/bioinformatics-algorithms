use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect: CAATC AATCC ATCCA TCCAA CCAAC
  let results = dna_replication::string_composition("CAATCCAAC", 5);

  println!("{}", results.join("\n"));
}
