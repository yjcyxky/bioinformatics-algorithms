use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect:
  // ```
  // GGCTTACCA
  // ```
  let patterns = vec![
    "CTTA",
    "ACCA",
    "TACC",
    "GGCT",
    "GCTT",
    "TTAC"
  ];

  let graph = dna_replication::de_bruijn_graph(&patterns, 4);

  println!("{:?}", graph.output_text(4));
}
