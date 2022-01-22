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
  let adjacent_lst = vec![
    vec![0, 3],
    vec![1, 0],
    vec![2, 1, 6],
    vec![3, 2],
    vec![4, 2],
    vec![5, 4],
    vec![6, 5, 8],
    vec![7, 9],
    vec![8, 7],
    vec![9, 6]
  ];
  let results = dna_replication::eulerian_cycle(&adjacent_lst, 6);

  println!("{}", results.iter().map(|item| item.to_string()).collect::<Vec<String>>().join("->"));
  
}
