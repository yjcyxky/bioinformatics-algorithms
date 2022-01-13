use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  println!("Neighbors: {:?}", dna_replication::immediate_neighbors("ACG"));
}
