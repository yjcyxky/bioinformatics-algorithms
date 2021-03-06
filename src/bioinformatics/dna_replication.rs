use itertools::Itertools;
use std::{collections::HashMap, usize};

fn find_max_value_with_ref(word_map: &HashMap<&str, usize>) -> Vec<String> {
  // It return different value when several word have the same frequency.
  // Returning String is better than returning &str.
  let mut max_value = 0;
  let mut keys = Vec::new();

  for (_, &v) in word_map.iter() {
    if v > max_value {
      max_value = v;
    }
  }

  for (&k, &v) in word_map.iter() {
    if v == max_value {
      keys.push(String::from(k));
    }
  }

  return keys;
}

pub fn find_max_value(word_map: &HashMap<String, usize>) -> Vec<String> {
  // It return different value when several word have the same frequency.
  // Returning String is better than returning &str.
  let mut max_value = 0;
  let mut keys = Vec::new();

  for (_, &v) in word_map.iter() {
    if v > max_value {
      max_value = v;
    }
  }

  for (k, &v) in word_map.iter() {
    if v == max_value {
      keys.push(k.clone());
    }
  }

  return keys;
}

fn count_kmer(text: &str, klen: usize) -> HashMap<&str, usize> {
  let mut word_map: HashMap<&str, usize> = HashMap::new();
  let length = text.len();

  for (idx, _) in text.chars().enumerate() {
    let count: usize;
    if idx + klen <= length {
      let kmer = &text[idx..(idx + klen)];
      match word_map.get(kmer) {
        Some(value) => {
          count = value + 1;
        }
        None => {
          count = 1;
        }
      }

      word_map.insert(kmer, count);
    }
  }

  return word_map;
}

fn find_pattern_by_freq(text: &str, klen: usize, nums: usize) -> Vec<String> {
  let word_map = count_kmer(text, klen);
  let mut keys = Vec::new();

  for (&k, &v) in word_map.iter() {
    if v >= nums {
      keys.push(String::from(k));
    }
  }

  return keys;
}

pub fn immediate_neighbors(pattern: &str) -> Vec<String> {
  let mut neighbors: Vec<String> = vec![];
  neighbors.push(String::from(pattern));

  for (index, char) in pattern.chars().enumerate() {
    for (_, base) in "ATCG".chars().enumerate() {
      if base != char {
        neighbors.push(
          String::from(&pattern[0..index]) + &base.to_string() + &pattern[index + 1..pattern.len()],
        );
      }
    }
  }

  return neighbors;
}

pub fn gen_kmer_patterns(k: usize) -> Vec<String> {
  let mut patterns: Vec<String> = vec![];

  for i in 0..k {
    if i == 0 {
      patterns = vec![
        String::from("A"),
        String::from("C"),
        String::from("G"),
        String::from("T"),
      ];
    } else {
      let mut temp: Vec<String> = vec![];
      for item in patterns.iter() {
        temp.push(String::from(&item[..]) + "A");
        temp.push(String::from(&item[..]) + "C");
        temp.push(String::from(&item[..]) + "G");
        temp.push(String::from(&item[..]) + "T");
      }

      patterns = temp;
    }
  }

  return patterns;
}

// --------------------- Functions for Rosalind ---------------------
// More details on https://rosalind.info/problems/ba1a/
pub fn count_pattern(text: &str, pattern: &str) -> u32 {
  let mut count = 0;
  let num = pattern.len();
  let length = text.len();

  // println!("Length: {}/{}", length, num);

  for (idx, _) in text.chars().enumerate() {
    if idx + num <= length {
      match &text[idx..(idx + num)] == pattern {
        true => count = count + 1,
        _ => continue,
      }
    }
  }

  return count;
}

// More details on https://rosalind.info/problems/ba1b/
pub fn find_frequent_word(text: &str, klen: usize) -> Vec<String> {
  let word_map = count_kmer(text, klen);

  return find_max_value_with_ref(&word_map);
}

pub fn complement(text: &str) -> String {
  let mut complement_text = String::from("");

  for (_, character) in text.chars().enumerate() {
    match character.to_ascii_uppercase() {
      'A' => {
        complement_text.push_str("T");
      }
      'T' => {
        complement_text.push_str("A");
      }
      'C' => {
        complement_text.push_str("G");
      }
      'G' => {
        complement_text.push_str("C");
      }
      _ => {}
    }
  }

  complement_text
}

// More details on https://rosalind.info/problems/ba1c/
pub fn reverse_complement(text: &str) -> String {
  let len = text.len();
  let mut reversed_text = vec![String::from(""); len];

  for (idx, character) in text.chars().enumerate() {
    match character.to_ascii_uppercase() {
      'A' => {
        reversed_text[len - idx - 1] = String::from("T");
      }
      'T' => {
        reversed_text[len - idx - 1] = String::from("A");
      }
      'C' => {
        reversed_text[len - idx - 1] = String::from("G");
      }
      'G' => {
        reversed_text[len - idx - 1] = String::from("C");
      }
      _ => {}
    }
  }

  reversed_text.join("")
}

// More details on https://rosalind.info/problems/ba1d/
pub fn find_pattern(text: &str, pattern: &str) -> Vec<usize> {
  let length = pattern.len();
  let count = text.len();
  let mut index_vec: Vec<usize> = vec![];

  for (idx, _) in text.chars().enumerate() {
    let max = idx + length;

    if max <= count {
      match pattern == &text[idx..max] {
        true => {
          index_vec.push(idx);
        }
        false => {}
      }
    }
  }

  return index_vec;
}

// More details on https://rosalind.info/problems/ba1e/
pub fn find_clump(text: &str, window_size: usize, len: usize, nums: usize) -> Vec<String> {
  let seq_len = text.len();
  let mut keys = Vec::new();

  for (idx, _) in text.chars().enumerate() {
    if idx + window_size <= seq_len && len <= window_size {
      let window_text = &text[idx..(idx + window_size)];
      keys.push(find_pattern_by_freq(window_text, len, nums));
    }
  }

  let uniq_keys = keys.concat().into_iter().unique().collect();

  return uniq_keys;
}

// More details on https://rosalind.info/problems/ba1f/
pub fn min_skew(text: &str, start: usize) -> Vec<Vec<i32>> {
  let length = text.len();
  let mut results: Vec<i32> = vec![];
  let mut pos: Vec<i32> = vec![];
  let mut count: i32 = 0;

  if start <= length {
    let first_half = &text[0..start];
    let second_half = &text[start..];

    for (idx, character) in second_half.chars().enumerate() {
      match character.to_ascii_uppercase() {
        'C' => {
          count = count - 1;
          results.push(count);
          pos.push((start + idx) as i32);
        }
        'G' => {
          count = count + 1;
          results.push(count);
          pos.push((start + idx) as i32);
        }
        _ => {
          results.push(count);
          pos.push((start + idx) as i32);
        }
      }
    }

    for (idx, character) in first_half.chars().enumerate() {
      match character.to_ascii_uppercase() {
        'C' => {
          count = count - 1;
          results.push(count);
          pos.push(idx as i32);
        }
        'G' => {
          count = count + 1;
          results.push(count);
          pos.push(idx as i32);
        }
        _ => {
          results.push(count);
          pos.push(idx as i32);
        }
      }
    }

    return vec![results, pos];
  } else {
    return vec![];
  }
}

// More details on https://rosalind.info/problems/ba1g/
// TODO: It will be failed, when the length of str2 is larger than the str1.
pub fn hamming_distance(str1: &str, str2: &str) -> usize {
  let mut distance = 0;

  for (index, c) in str1.chars().enumerate() {
    if c.to_string() != str2[index..index + 1] {
      distance += 1;
    }
  }

  return distance;
}

// More details on https://rosalind.info/problems/ba1h/
pub fn count_pattern_with_mismatches(text: &str, pattern: &str, d: usize) -> Vec<usize> {
  let num = pattern.len();
  let length = text.len();
  let mut start_poses: Vec<usize> = vec![];

  // println!("Length: {}/{}", length, num);

  for (idx, _) in text.chars().enumerate() {
    if idx + num <= length {
      match hamming_distance(&text[idx..(idx + num)], pattern) <= d {
        true => start_poses.push(idx),
        _ => continue,
      }
    }
  }

  return start_poses;
}

// More details on https://rosalind.info/problems/ba1i/
pub fn find_freq_pattern_mis(text: &str, k: usize, d: usize) -> HashMap<String, usize> {
  let patterns: Vec<String> = gen_kmer_patterns(k);
  let mut freq_map: HashMap<String, usize> = HashMap::new();

  for i in patterns.iter() {
    for (j, _) in text.chars().enumerate() {
      if j + k <= text.len() {
        match hamming_distance(&i[..], &text[j..(j + k)]) <= d {
          true => {
            let count: usize;
            match freq_map.get(&i[..]) {
              Some(value) => {
                count = value + 1;
              }
              None => {
                count = 1;
              }
            }
            freq_map.insert(i.clone(), count);
          }
          _ => continue,
        }
      }
    }
  }

  return freq_map;
}

// More details on https://rosalind.info/problems/ba1j/
#[allow(mutable_borrow_reservation_conflict)]
pub fn find_freq_pattern_misrev(text: &str, k: usize, d: usize) -> HashMap<String, usize> {
  let freq_map: HashMap<String, usize> = find_freq_pattern_mis(text, k, d);

  let reversed_text = reverse_complement(text);
  let mut reversed_freq_map = find_freq_pattern_mis(&reversed_text[..], k, d);

  for (_, key) in freq_map.keys().enumerate() {
    match reversed_freq_map.get(&key[..]) {
      Some(value) => {
        reversed_freq_map.insert(key.clone(), value + freq_map.get(&key[..]).unwrap());
      }
      None => {}
    }
  }

  return reversed_freq_map;
}

// More details on https://rosalind.info/problems/ba1k/
pub fn compute_freq_array(text: &str, k: usize) -> Vec<usize> {
  let patterns: Vec<String> = gen_kmer_patterns(k);
  let mut pattern_map: HashMap<&str, usize> = HashMap::new();
  let mut frequency = vec![];
  let length = text.len();

  for item in patterns.iter() {
    pattern_map.insert(&item[..], 0);
  }

  for (index, _) in text.chars().enumerate() {
    let count: usize;
    if index + k <= length {
      match pattern_map.get(&text[index..(index + k)]) {
        Some(value) => {
          count = value + 1;
        }
        None => {
          count = 1;
        }
      }

      pattern_map.insert(&text[index..(index + k)], count);
    }
  }

  for item in patterns.iter() {
    frequency.push(pattern_map.get(&item[..]).unwrap().to_owned());
  }

  return frequency;
}

pub fn pattern2number_slow(pattern: &str) -> usize {
  let k: usize = pattern.len();
  let pattern_array: Vec<String> = gen_kmer_patterns(k);

  for (idx, item) in pattern_array.iter().enumerate() {
    if &item[..] == pattern {
      return idx;
    }
  }

  // Never run
  return pattern_array.len();
}

// More details on https://rosalind.info/problems/ba1l/
pub fn pattern2number(pattern: &str) -> usize {
  let k: usize = pattern.len();
  let base2number = |base| match base {
    "A" => return Ok(0),
    "C" => return Ok(1),
    "G" => return Ok(2),
    "T" => return Ok(3),
    _ => return Err("Cannot identify the base."),
  };

  if k == 1 {
    return base2number(pattern).unwrap();
  } else {
    return 4 * pattern2number(&pattern[0..k - 1]) + base2number(&pattern[k - 1..k]).unwrap();
  }
}

pub fn number2pattern(number: usize) -> String {
  let number2base = |number| match number {
    0 => return Ok("A"),
    1 => return Ok("C"),
    2 => return Ok("G"),
    3 => return Ok("T"),
    _ => return Err("Cannot identify the base."),
  };

  if number <= 3 {
    return number2base(number).unwrap().to_string();
  } else {
    let rem = number % 4;
    let div = number / 4;

    if div <= 3 {
      return number2base(div).unwrap().to_string() + number2base(rem).unwrap();
    } else {
      return number2pattern(div) + number2base(rem).unwrap();
    }
  }
}

// More details on https://rosalind.info/problems/ba1m/
pub fn number2kmer_pattern(number: usize, k: usize) -> String {
  let pattern = number2pattern(number);
  let gap: i32 = k as i32 - pattern.len() as i32;
  let rep_a = |num| {
    let mut str = String::from("A");
    for _ in 1..num {
      str += "A";
    }
    return str;
  };

  if gap > 0 {
    return rep_a(gap) + &pattern[..];
  } else {
    return pattern;
  }
}

// More details on https://rosalind.info/problems/ba1n/
pub fn neighbors(pattern: &str, d: usize) -> Vec<String> {
  if d == 0 {
    return vec![pattern.to_string()];
  }

  if pattern.len() == 1 {
    return vec![
      "A".to_string(),
      "C".to_string(),
      "G".to_string(),
      "T".to_string(),
    ];
  }

  let mut nb_set: Vec<String> = vec![];
  let suffix_nb_set = neighbors(&pattern[1..], d);
  for item in suffix_nb_set.iter() {
    if hamming_distance(&item[..], &pattern[1..]) < d {
      for base in ["A", "C", "G", "T"] {
        nb_set.push(String::from(base) + &item[..]);
      }
    } else {
      nb_set.push(String::from(&pattern[0..1]) + &item[..]);
    }
  }
  return nb_set;
}

fn is_matched(text: &str, kmer: &str, d: usize) -> bool {
  let length = kmer.len();
  for (idx, _) in text.chars().enumerate() {
    if idx + length <= text.len() {
      if hamming_distance(&text[idx..(idx + length)], kmer) <= d {
        return true;
      }
    }
  }

  return false;
}

fn all_is_matched(dna_array: &Vec<&str>, kmer: &str, d: usize) -> bool {
  for dna in dna_array.iter() {
    if !is_matched(&dna[..], kmer, d) {
      return false;
    }
  }

  return true;
}

// More details on https://rosalind.info/problems/ba2a/
pub fn motif_enumeration(dna_array: Vec<&str>, k: usize, d: usize) -> Vec<String> {
  let freq_patterns = find_freq_pattern_mis(&dna_array[0], k, d);
  let candidates = freq_patterns.keys();
  let mut results: Vec<String> = vec![];

  for candidate in candidates {
    if all_is_matched(&dna_array, &candidate[..], d) {
      results.push(candidate.clone());
    }
  }

  return results;
}

fn min_distance(text: &str, pattern: &str) -> usize {
  let mut min: usize = pattern.len();
  let length = text.len();
  let k = pattern.len();

  for (idx, _) in text.chars().enumerate() {
    if (idx + k) <= length {
      let d = hamming_distance(pattern, &text[idx..idx + k]);
      if d == 0 {
        return 0;
      } else if min > d {
        min = d;
      }
    }
  }

  return min;
}

fn sum_distance(dna_array: &Vec<&str>, pattern: &str) -> usize {
  let mut sum: usize = 0;
  for dna in dna_array {
    sum += min_distance(dna, pattern);
  }

  return sum;
}

pub fn median_str(dna_array: Vec<&str>, k: usize) -> String {
  let pattern_array: Vec<String> = gen_kmer_patterns(k);
  let mut min_sum: usize = dna_array.len() * k;
  let mut min_pattern = String::from("");

  for pattern in pattern_array {
    let sum = sum_distance(&dna_array, &pattern[..]);
    if sum <= min_sum {
      min_sum = sum;
      min_pattern = pattern.clone();
    }

    // println!("Min Distance: {}/{}/{}", &pattern[..], sum, min_sum);
  }

  return min_pattern;
}

pub fn string_composition(text: &str, k: usize) -> Vec<String> {
  let mut kmer_set: Vec<String> = vec![];
  let length = text.len();

  for (idx, _) in text.chars().enumerate() {
    if (idx + k) <= length {
      kmer_set.push(text[idx..idx + k].to_string());
    }
  }

  return kmer_set;
}

// More details on https://rosalind.info/problems/ba3b/
pub fn genome_path(dna_array: &[&str]) -> String {
  let length = dna_array.len();
  if length == 1 {
    return dna_array[0].to_string();
  } else {
    let path = genome_path(&dna_array[0..length - 1]);
    let last = dna_array[length - 1];
    let str_len = last.len();
    return path + &last[(str_len - 1)..str_len];
  }
}

// More details on https://rosalind.info/problems/ba3c/
pub fn overlap_graph(kmers: &[&str]) -> Vec<Vec<String>> {
  let suffix = |pattern: &str| return pattern[1..].to_string();
  let prefix = |pattern: &str| return pattern[0..pattern.len() - 1].to_string();
  let mut results: Vec<Vec<String>> = vec![];

  for (i, kmer) in kmers.iter().enumerate() {
    for (j, pattern) in kmers.iter().enumerate() {
      if i != j && suffix(kmer) == prefix(pattern) {
        results.push(vec![kmer.to_string(), pattern.to_string()]);
      }
    }
  }

  return results;
}

fn gen_kmers(text: &str, k: usize) -> Vec<String> {
  let mut results: Vec<String> = vec![];
  for (idx, _) in text.chars().enumerate() {
    if (idx + k) <= text.len() {
      results.push(String::from(&text[idx..idx + k]));
    }
  }

  return results;
}

// More details on https://rosalind.info/problems/ba3d/
pub fn de_bruijn(text: &str, k: usize) -> Vec<Vec<String>> {
  let mut results: Vec<Vec<String>> = vec![];

  for (idx, _) in text.chars().enumerate() {
    if idx + k <= text.len() {
      let cstr = &text[idx..idx + k - 1];
      let nstr = &text[idx + 1..idx + k];
      match results.iter_mut().find(|item| &item[0][..] == cstr) {
        Some(value) => {
          value.push(nstr.to_string());
        }
        None => {
          results.push(vec![cstr.to_string(), nstr.to_string()]);
        }
      }
    }
  }

  return results;
}

fn de_bruijn_mut(text: &str, k: usize, results: &mut Vec<Vec<String>>) {
  for (idx, _) in text.chars().enumerate() {
    if idx + k <= text.len() {
      let cstr = &text[idx..idx + k - 1];
      let nstr = &text[idx + 1..idx + k];
      match results.iter_mut().find(|item| &item[0][..] == cstr) {
        Some(value) => {
          value.push(nstr.to_string());
        }
        None => {
          results.push(vec![cstr.to_string(), nstr.to_string()]);
        }
      }
    }
  }
}

// More details on https://rosalind.info/problems/ba3e/
pub fn kmer_de_bruijn(kmers: &Vec<&str>) -> Vec<Vec<String>> {
  let mut results: Vec<Vec<String>> = vec![];
  for kmer in kmers {
    de_bruijn_mut(kmer, kmer.len(), &mut results);
  }

  return results;
}

pub fn make_graph(adjacent_lst: &Vec<Vec<&str>>) -> HashMap<String, Vec<String>> {
  let mut graph: HashMap<String, Vec<String>> = HashMap::new();
  for (_, item) in adjacent_lst.iter().enumerate() {
    graph.insert(
      item[0].to_string(),
      item[1..].iter().map(|item| item.to_string()).collect(),
    );
  }

  return graph;
}

fn find_idx(v: &Vec<usize>, graph: &HashMap<usize, Vec<usize>>) -> i32 {
  let length = v.len();
  for idx in 0..length {
    let index = length - 1 - idx;
    match graph.get(&v[index]) {
      Some(_) => {
        return index as i32;
      }
      _ => {}
    }
  }

  return -1;
}

pub fn eulerian_cycle(adjacent_lst: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
  let mut cycle: Vec<usize> = vec![start];
  let mut new_start: usize = start;
  let mut rest: Vec<usize> = vec![];
  let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
  for (_, item) in adjacent_lst.iter().enumerate() {
    graph.insert(
      item[0],
      item[1..].iter().map(|item| item.to_owned()).collect(),
    );
  }

  let find_valid_idx = |items: &Vec<usize>, start: usize| {
    for (idx, item) in items.iter().enumerate() {
      if item.to_owned() != start {
        return idx;
      }
    }

    return 0;
  };

  loop {
    if !graph.is_empty() {
      if let Some(value) = graph.get(&new_start) {
        // It's a cycle but not a finished.
        if value.len() > 1 {
          let valid_idx = find_valid_idx(value, start);
          let end = value[valid_idx];
          let new_value = value
            .clone()
            .into_iter()
            .filter(|&item| item != end)
            .collect();
          graph.insert(new_start, new_value);
          cycle.push(end);
          new_start = end;
        } else {
          let end = value[0];
          graph.remove(&new_start);
          cycle.push(end);
          new_start = end;
        }
      } else {
        // New Start
        let idx = find_idx(&cycle, &graph);
        if idx >= 0 {
          if rest.len() > 0 {
            let new_rest = cycle[idx as usize + 1..].iter().map(|&item| item).collect();
            rest = [new_rest, rest].concat();
          } else {
            rest = cycle[idx as usize + 1..].iter().map(|&item| item).collect();
          }
          cycle.truncate(idx as usize + 1);
          new_start = cycle[idx as usize];
        } else {
          // TODO: Not valid cycle.
          return cycle;
        }
      }
    } else {
      if rest.len() > 0 {
        cycle = [cycle, rest].concat();
      }
      break;
    }
  }

  return cycle;
}

pub fn compute_degree(adjacent_lst: &Vec<Vec<usize>>) -> HashMap<usize, Vec<usize>> {
  let mut results: HashMap<usize, Vec<usize>> = HashMap::new();
  for item in adjacent_lst {
    let length = item.len();
    if let Some(v) = results.get_mut(&item[0]) {
      v[0] = length - 1;
    } else {
      results.insert(item[0], vec![length - 1, 0]);
    }

    for &i in item[1..].iter() {
      if let Some(v) = results.get_mut(&i) {
        v[1] += 1;
      } else {
        results.insert(i, vec![0, 1]);
      }
    }
  }

  return results;
}

pub fn eulerian_path(adjacent_lst: &Vec<Vec<usize>>) -> Vec<usize> {
  let degree = compute_degree(adjacent_lst);
  let mut start: usize = 0;
  // let mut end: usize = 0;
  for (k, v) in degree {
    if v[0] > v[1] {
      start = k;
    }

    // if v[0] < v[1] {
    //   end = k;
    // }
  }

  return eulerian_cycle(&adjacent_lst, start);
}

#[derive(Debug)]
pub struct Graph {
  nodes: Vec<String>,
  edges: Vec<Vec<usize>>,
}

impl Graph {
  pub fn new() -> Graph {
    Graph {
      nodes: vec![],
      edges: vec![],
    }
  }

  pub fn get_edges(&self) -> Vec<Vec<usize>> {
    return self.edges.clone();
  }

  pub fn get_nodes(&self) -> Vec<String> {
    return self.nodes.clone();
  }

  fn find_node(&self, node: &str) -> i32 {
    for (i, v) in self.nodes.iter().enumerate() {
      if &v[..] == node {
        return i as i32;
      }
    }

    return -1;
  }

  fn find_edge(&self, start: usize) -> i32 {
    for (i, v) in self.edges.iter().enumerate() {
      if v[0] == start {
        return i as i32;
      }
    }

    return -1;
  }

  fn add_node(&mut self, node: &str) -> usize {
    let idx = self.find_node(node);
    if idx >= 0 {
      return idx as usize;
    } else {
      self.nodes.push(node.to_string());
      return self.nodes.len() - 1;
    }
  }

  pub fn add_edge(&mut self, start: &str, end: &str) {
    let start_idx = self.add_node(start);
    let end_idx = self.add_node(end);

    let edge_idx = self.find_edge(start_idx);
    if edge_idx >= 0 {
      self.edges[edge_idx as usize].push(end_idx);
    } else {
      self.edges.push(vec![start_idx, end_idx]);
    }
  }

  pub fn output_text(&self, k: usize) -> String {
    let path = eulerian_path(&self.edges);
    let mut text = String::from(&self.nodes[path[0]][..]);

    for i in 1..path.len() {
      text += &self.nodes[path[i]][k - 2..];
    }

    return text;
  }

  pub fn output_paired_text(&self, k: usize, d: usize) -> String {
    let path = eulerian_path(&self.edges);
    let mut text1 = String::from(&self.nodes[path[0]][..k - 1]);
    let mut text2 = String::from(&self.nodes[path[0]][k..]);

    for i in 1..path.len() {
      text1 += &self.nodes[path[i]][k - 2..k - 1];
      text2 += &self.nodes[path[i]][2 * k - 2..];
    }

    let end = k + d;

    return format!("{}{}", &text1[..end], text2);
  }
}

pub fn de_bruijn_graph(patterns: &Vec<&str>, k: usize) -> Graph {
  let mut graph = Graph::new();

  for &pattern in patterns {
    let prefix = &pattern[0..k - 1];
    let suffix = &pattern[1..];
    graph.add_edge(prefix, suffix);
  }

  return graph;
}

pub fn gen_k_universal_patterns(k: usize) -> Vec<String> {
  let mut patterns: Vec<String> = vec![];

  for i in 0..(2 as usize).pow(k as u32) {
    patterns.push(format!("{:0width$b}", i, width=k));
  }

  return patterns;
}

pub fn k_universal_cycle(k: usize) -> String {
  let patterns = gen_k_universal_patterns(k);
  let graph = de_bruijn_graph(&patterns.iter().map(|item| &item[..]).collect(), k);
  return graph.output_text(k);
}

pub fn read_pairs(patterns: &Vec<&str>, k: usize) -> Graph {
  let mut graph = Graph::new();

  for &pattern in patterns {
    let read1 = &pattern[0..k];
    let read2 = &pattern[k + 1..];
    let read1_prefix = &read1[0..k - 1];
    let read1_suffix = &read1[1..];
    let read2_prefix = &read2[0..k - 1];
    let read2_suffix = &read2[1..];
    let prefix = format!("{}|{}", read1_prefix, read2_prefix);
    let suffix = format!("{}|{}", read1_suffix, read2_suffix);
    graph.add_edge(&prefix[..], &suffix[..]);
  }

  return graph;
}