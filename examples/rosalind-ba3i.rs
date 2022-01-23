use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect:
  // ```
  // 0001001101011110
  // ```
  let k = 4;
  let results = dna_replication::k_universal_cycle(k);
  println!("K Universal String: {:?}", &results[..results.len() - k + 1]);
}
