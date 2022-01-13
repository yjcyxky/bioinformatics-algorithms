use bioinformatics_algorithms::bioinformatics::{dna_replication, fasta, util};

#[warn(unused_must_use)]
fn main() {
  // E.coli Fasta
  let fasta: String = fasta::read_fasta("./examples/data/E.coli.fasta");
  let results = dna_replication::min_skew(&fasta[..], 0);
  util::write_csv(results, "./examples/results/min-skew.csv").unwrap();
}
