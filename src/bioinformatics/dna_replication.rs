use std::collections::HashMap;
use itertools::Itertools;

// More details on https://rosalind.info/problems/ba1a/
pub fn count_pattern(text: &str, pattern: &str) -> u32 {
  let mut count = 0;
  let num = pattern.len();
  let length = text.len();

  // println!("Length: {}/{}", length, num);

  for (idx, _) in text.chars().enumerate() {
    let max_len = idx + num;
    if max_len < length {
      match &text[idx..(idx + num)] == pattern {
        true => count = count + 1,
        _ => continue,
      }
    }
  }

  return count;
}

fn find_max_value(word_map: &HashMap<&str, usize>) -> Vec<String> {
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

fn count_kmer(text: &str, klen: usize) -> HashMap<&str, usize> {
  let mut word_map: HashMap<&str, usize> = HashMap::new();
  let length = text.len();

  for (idx, _) in text.chars().enumerate() {
    let mut count = 0;
    if idx + klen < length {
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

pub fn find_frequent_word(text: &str, klen: usize) -> Vec<String> {
  let word_map = count_kmer(text, klen);

  return find_max_value(&word_map);
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

// TODO: It will be failed, when the length of str2 is larger than the str1.
pub fn hamming_distance(str1: &str, str2: &str) -> usize {
  let mut distance = 0;

  for (index, c) in str1.chars().enumerate() {
    if c.to_string() != str2[index..index+1] {
      distance += 1;
    }
  }

  return distance;
}

pub fn count_pattern_with_mismatches(text: &str, pattern: &str, d: usize) -> Vec<usize> {
  let mut count = 0;
  let num = pattern.len();
  let length = text.len();
  let mut start_poses: Vec<usize> = vec![];

  // println!("Length: {}/{}", length, num);

  for (idx, _) in text.chars().enumerate() {
    let max_len = idx + num;
    if max_len < length {
      match hamming_distance(&text[idx..(idx + num)], pattern) <= d {
        true => start_poses.push(idx),
        _ => continue,
      }
    }
  }

  return start_poses;
}