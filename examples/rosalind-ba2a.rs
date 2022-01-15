use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect: ATA ATT GTT TTT
  let results = dna_replication::motif_enumeration(vec!["ATTTGGC", "TGCCTTA", "CGGTATC", "GAAAATT"], 3, 1);

  println!("{}", results.join(" "));
}
