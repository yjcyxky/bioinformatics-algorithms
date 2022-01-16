use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect: CAATCCAAC
  // Given: dna_array - A sequence of k-mers Pattern1, ... , Patternn such that the last k - 1 symbols of Patterni are equal to the first k - 1 symbols of Patterni+1 for i from 1 to n-1.
  let results = dna_replication::genome_path(&["CAATC", "AATCC", "ATCCA", "TCCAA", "CCAAC"]);

  println!("{}", results);
}
