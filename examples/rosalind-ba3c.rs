use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect:
  // ```
  // AGGCA -> GGCAT
  // CATGC -> ATGCG
  // GCATG -> CATGC
  // GGCAT -> GCATG
  // ```
  let results = dna_replication::overlap_graph(&["ATGCG", "GCATG", "CATGC", "AGGCA", "GGCAT"]);

  for item in results.iter() {
    println!("{}", item.join(" -> "));
  }
}
