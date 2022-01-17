use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect:
  // ```
  // AAG -> AGA
  // AGA -> GAT
  // ATT -> TTC
  // CTA -> TAC
  // CTC -> TCT
  // GAT -> ATT
  // TCT -> CTA,CTC
  // TTC -> TCT
  // ```
  let results = dna_replication::de_bruijn("AAGATTCTCTAC", 4);

  for item in results.iter() {
    if item.len() == 2 {
      println!("{}", item.join(" -> "));
    } else {
      println!("{} -> {}", item[0], &item[1..].join(","));
    }
  }
}
