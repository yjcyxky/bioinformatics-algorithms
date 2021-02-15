use algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  println!(
    "Reverse Complement: {}",
    dna_replication::reverse_complement("ATGATCAAG")
  );
}
