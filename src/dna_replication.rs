use std::collections::HashMap;

pub fn count_pattern(text: &str, pattern: &str) -> u32 {
  let mut count = 0;
  let num = pattern.len();
  let length = text.len();

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

pub fn find_max_value(word_map: &HashMap<&str, usize>) -> String {
  // It return different value when several word have the same frequency.
  // Returning String is better than returning &str.
  let mut max_value = 0;
  let mut key = "";

  for (&k, &v) in word_map.iter() {
    if v > max_value {
      max_value = v;
      key = k;
    }
  }

  return String::from(key);
}

pub fn find_frequent_word(text: &str, k: usize) -> String {
  let mut word_map: HashMap<&str, usize> = HashMap::new();
  let length = text.len();

  for (idx, _) in text.chars().enumerate() {
    let mut count = 0;
    if idx + k < length {
      let kmer = &text[idx..(idx + k)];
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

  return find_max_value(&word_map);
}

pub fn reverse_complement(text: &str) -> String {
  let mut reversed_text = String::from("");

  for (_, character) in text.chars().enumerate() {
    match character.to_ascii_uppercase() {
      'A' => {
        reversed_text.push_str("T");
      }
      'T' => {
        reversed_text.push_str("A");
      }
      'C' => {
        reversed_text.push_str("G");
      }
      'G' => {
        reversed_text.push_str("C");
      }
      _ => {}
    }
  }

  reversed_text
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
