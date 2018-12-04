extern crate clap;
extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App};

#[derive(Debug)]
struct Rect {
    id:   usize,
    xpos: usize,
    ypos: usize,
    xdim: usize,
    ydim: usize,
}

// find number of square inches with more than one claim by building an
// occupancy map
fn find_double_occupancy(claims: &[Rect]) -> usize {
    let mut oc_grid = vec![0usize; 1000 * 1000];
    fn idx(x: usize, y: usize) -> usize { y*1000 + x }

    for claim in claims {
        for x in claim.xpos .. claim.xpos + claim.xdim {
            for y in claim.ypos .. claim.ypos + claim.ydim {
                oc_grid[idx(x,y)] += 1;
            }
        }
    }

    let mut count = 0;
    for grid_square in oc_grid {
        if grid_square > 1 {
            count += 1;
        }
    }
    count
}

// check if claim overlaps another
fn check_overlap(a: &Rect, b: &Rect) -> bool {
    let ax_tl = a.xpos;
    let ax_br = a.xpos + a.xdim - 1;
    let bx_tl = b.xpos;
    let bx_br = b.xpos + b.xdim - 1;
    let ay_tl = a.ypos;
    let ay_br = a.ypos + a.ydim - 1;
    let by_tl = b.ypos;
    let by_br = b.ypos + b.ydim - 1;
    !(((ax_tl > bx_br) || (bx_tl > ax_br)) ||
      ((ay_tl > by_br) || (by_tl > ay_br)))
}

// find a claim that has no overlap with other claims
//
// we do this by testing each claim for overlap against other claims. this could
// be sped up by removing colliding claims on discovery but thats more work
fn find_solo_claim(claims: &[Rect]) -> Option<usize> {
    let mut solo_claim: Option<usize> = None;
    'outer: for a in claims {
        'inner: for b in claims {
            if a.id == b.id { continue 'inner };
            if check_overlap(a,b) { continue 'outer };
        }
        solo_claim = Some(a.id);
        println!("{:?}",a);
    }
    solo_claim
}

fn main() {
    let matches = App::new("c3")
                        .author("S Donnan <sdonnan@fastmail.com>")
                        .about("Solves AoC 2018 Challenge 3")
                        .args_from_usage("<input> 'Textual puzzle input'")
                        .get_matches();

    let file = File::open(matches.value_of("input").unwrap()).unwrap();
    let reader = BufReader::new(file);

    // create parsing regex
    let re = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();
    // create vector to put parsed inputs into
    let mut inputs: Vec<Rect> = Vec::new();

    // parse inputs
    for line in reader.lines() {
       match re.captures(&line.unwrap()) {
           Some(caps) => {
               inputs.push(
                   Rect {
                       id:   caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                       xpos: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                       ypos: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                       xdim: caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                       ydim: caps.get(5).unwrap().as_str().parse::<usize>().unwrap(),
                   }
               )
           }
           None => {
               println!("Error parsing line");
           }
       }
    }

    println!("Part1: {}", find_double_occupancy(&inputs));
    println!("Part2: {:?}", find_solo_claim(&inputs));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_1() {
        // #1 @ 1,3: 4x4
        // #2 @ 3,1: 4x4
        // #3 @ 5,5: 2x2
        assert_eq!(4, find_double_occupancy(&vec![
                        Rect {id: 1, xpos: 1, ypos: 3, xdim: 4, ydim: 4},
                        Rect {id: 2, xpos: 3, ypos: 1, xdim: 4, ydim: 4},
                        Rect {id: 3, xpos: 5, ypos: 5, xdim: 2, ydim: 2},
        ]));
    }

    #[test]
    fn test_example_2() {
        // #1 @ 1,3: 4x4
        // #2 @ 3,1: 4x4
        // #3 @ 5,5: 2x2
        assert_eq!(Some(3), find_solo_claim(&vec![
                        Rect {id: 1, xpos: 1, ypos: 3, xdim: 4, ydim: 4},
                        Rect {id: 2, xpos: 3, ypos: 1, xdim: 4, ydim: 4},
                        Rect {id: 3, xpos: 5, ypos: 5, xdim: 2, ydim: 2},
        ]));
    }

    #[test]
    fn test_overlap_check() {
        let a = Rect {id: 1, xpos: 1, ypos: 3, xdim: 4, ydim: 4};
        let b = Rect {id: 2, xpos: 3, ypos: 1, xdim: 4, ydim: 4};
        let c = Rect {id: 3, xpos: 5, ypos: 5, xdim: 2, ydim: 2};
        let d = Rect {id: 570, xpos: 150, ypos: 14, xdim: 23, ydim: 22};
        let e = Rect {id: 845, xpos: 157, ypos: 13, xdim: 14, ydim: 24};
        assert_eq!(true,  check_overlap(&a, &b));
        assert_eq!(true,  check_overlap(&b, &a));
        assert_eq!(false, check_overlap(&a, &c));
        assert_eq!(true, check_overlap(&d, &e));
    }
}
