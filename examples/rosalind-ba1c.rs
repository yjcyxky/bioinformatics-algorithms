use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect "ACCGGGTTTT"
  println!(
    "Reverse Complement: {}",
    dna_replication::reverse_complement("AAAACCCGGT")
  );
}
