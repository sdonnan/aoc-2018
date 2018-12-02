extern crate clap;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::BTreeMap;
use clap::{App};

// Find checksum of all box ids
// This function is faster than the hash table version but assumes that inputs
// strings are ASCII lowercase chars and thus is more fragile
#[allow(dead_code)]
fn checksum (ids: &[String]) -> usize {
    let mut twice = 0usize;
    let mut thrice = 0usize;
    for id in ids {
        let mut char_count = [0usize; 26];
        // count duplicate char_count
        for b in id.bytes() {
            let idx = (b - b'a') as usize;
            char_count[idx] = char_count[idx] + 1;
        }
        // check for any that occur exactly twice or thrice
        let mut twice_flag = false;
        let mut thrice_flag = false;
        for count in char_count.iter() {
            if !twice_flag && *count == 2 {
                twice += 1;
                twice_flag = true;
            }
            if !thrice_flag && *count == 3 {
                thrice += 1;
                thrice_flag = true;
            }
            if twice_flag && thrice_flag {break;}
        }
    }
    thrice * twice
}

// Find checksum of all box ids using a hash table
fn checksum_ht (ids: &[String]) -> usize {
    let mut twice = 0usize;
    let mut thrice = 0usize;
    for id in ids {
        let mut char_count: BTreeMap<char, usize> = BTreeMap::new();
        // count duplicate char_count
        for c in id.chars() {
            let count = char_count.entry(c).or_insert(0);
            *count += 1;
        }
        // check for any that occur exactly twice or thrice
        let mut twice_flag = false;
        let mut thrice_flag = false;
        for (_, value) in char_count.iter() {
            if !twice_flag && *value == 2 {
                twice += 1;
                twice_flag = true;
            }
            if !thrice_flag && *value == 3 {
                thrice += 1;
                thrice_flag = true;
            }
            if twice_flag && thrice_flag {break;}
        }
    }
    thrice * twice
}

// find two ids one letter apart and return them. None if no pairs and if there
// are multiple pairs only the first is returned
//
// the approach is to sort the list (which is O(n log n)) and then compare
// neighbors rather than comparing each possible pair in the list
fn find_id_pair(ids: &[String]) -> Option<(String, String)> {
    let mut sorted_ids = ids.to_vec();
    sorted_ids.sort_unstable();
    'compare: for idx in 1..(sorted_ids.len() - 1) {
        // get items from sorted list as slices
        let a = sorted_ids[idx - 1].as_str();
        let b = sorted_ids[idx].as_str();
        // check chars
        let mut diff_idx: Option<usize> = None;
        for ((ac, bc), idx) in a.chars().zip(b.chars()).zip(0..) {
            // mismatch?
            if ac != bc {
                // if we already have a difference then this cant be it
                if diff_idx != None {continue 'compare;}
                diff_idx = Some(idx);
            }
        }
        // if we make it here then either the ids are the same or have a single
        // difference
        match diff_idx {
            Some(_) => {
                return Some((a.to_string(), b.to_string()));
            }
            None => {}
        };
    }
    Option::None
}

fn main() {
    let matches = App::new("c2")
                        .author("S Donnan <sdonnan@fastmail.com>")
                        .about("Solves AoC 2018 Challenge 2")
                        .args_from_usage("<input> 'Textual puzzle input'")
                        .get_matches();

    let file = File::open(matches.value_of("input").unwrap()).unwrap();
    let reader = BufReader::new(file);

    // parse inputs
    let inputs = reader.lines()
                       .map(|result| {
                           result.unwrap()
                                 .trim()
                                 .to_string()
                       })
                       .collect::<Vec<String>>();

    println!("Part1: {}", checksum_ht(&inputs));
    match find_id_pair(&inputs) {
        Some((a, b)) => {
            println!("Part2: Pair IDs\n {}\n {}", a, b);
        }
        None => {
            println!("Part2: No matches found")
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(12,checksum(&vec!["abcdef".to_string(),
                                     "bababc".to_string(),
                                     "abbcde".to_string(),
                                     "abcccd".to_string(),
                                     "aabcdd".to_string(),
                                     "abcdee".to_string(),
                                     "ababab".to_string()]));
    }

    #[test]
    fn test_example_1_ht() {
        assert_eq!(12,checksum_ht(&vec!["abcdef".to_string(),
                                        "bababc".to_string(),
                                        "abbcde".to_string(),
                                        "abcccd".to_string(),
                                        "aabcdd".to_string(),
                                        "abcdee".to_string(),
                                        "ababab".to_string()]));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Some(("fghij".to_string(), "fguij".to_string())),
                   find_id_pair(
                        &vec!["abcde".to_string(),
                              "fghij".to_string(),
                              "klmno".to_string(),
                              "pqrst".to_string(),
                              "fguij".to_string(),
                              "axcye".to_string(),
                              "wvxyz".to_string()]));
    }
}
