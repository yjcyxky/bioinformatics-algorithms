use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect 1, 3, 9
  println!(
    "Pattern Match: {:?}",
    dna_replication::find_pattern("GATATATGCATATACTT", "ATAT")
  );
}
