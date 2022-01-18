use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect:
  // ```
  // AGG -> GGG
  // CAG -> AGG,AGG
  // GAG -> AGG
  // GGA -> GAG
  // GGG -> GGA,GGG
  // ```
  let results = dna_replication::kmer_de_bruijn(&vec!["GAGG", "CAGG", "GGGG", "GGGA", "CAGG", "AGGG", "GGAG"]);

  for item in results.iter() {
    if item.len() == 2 {
      println!("{}", item.join(" -> "));
    } else {
      println!("{} -> {}", item[0], &item[1..].join(","));
    }
  }
}
