use bioinformatics_algorithms::bioinformatics::dna_replication;

#[warn(unused_must_use)]
fn main() {
  // Expect:
  // ```
  // 6->7->8->9->6->3->0->2->1->3->4
  // ```
  let adjacent_lst = vec![
    vec![0, 2],
    vec![1, 3],
    vec![2, 1],
    vec![3, 0, 4],
    vec![6, 3, 7],
    vec![7, 8],
    vec![8, 9],
    vec![9, 6],
  ];

  let results = dna_replication::eulerian_path(&adjacent_lst);
  println!("{}", results.iter().map(|item| item.to_string()).collect::<Vec<String>>().join("->"));
}
