extern crate clap;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
use clap::{App};

// Find repeated frequency for part 2
fn find_repeat(freq_adjustments: &[isize]) -> isize {
    let mut visited = HashSet::new();
    let mut current_freq = 0isize;
    visited.insert(current_freq);
    'searchloop: loop {
        for f in freq_adjustments {
            current_freq = current_freq + f;
            if visited.contains(&current_freq) {
                break 'searchloop;
            }
            visited.insert(current_freq);
        }
    }
    current_freq
}

fn main() {
    let matches = App::new("c1")
                        .author("S Donnan <sdonnan@fastmail.com>")
                        .about("Solves AoC 2018 Challenge 1")
                        .args_from_usage("<input> 'Textual puzzle input'")
                        .get_matches();

    let file = File::open(matches.value_of("input").unwrap()).unwrap();
    let reader = BufReader::new(file);

    // parse inputs
    let freq_adjustments = reader.lines()
                                 .map(|result| {
                                     result.unwrap()
                                           .trim()
                                           .parse::<isize>()
                                           .unwrap()
                                 })
                                 .collect::<Vec<_>>();

    println!("Part1: {}", freq_adjustments.iter().sum::<isize>());
    println!("Part1: {}", find_repeat(&freq_adjustments));
}

#[test]
fn test_example_2() {
    assert_eq!(find_repeat(&vec![7, 7, -2, -7, -4]), 14);
}

