#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static! {
    // #1 @ 1,3: 4x4
    static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
}

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    dx: u32,
    dy: u32,
}

fn main() {
    let claims = parse_claims("input.txt");
    let mut fabric = make_fabric(1000);
    lay_claims(&mut fabric, &claims);
    println!("Solution to day 3 part 1: {}", part1(&fabric));
    println!("Solution to day 3 part 2: {}", part2(&fabric, &claims));
}

fn part1(fabric: &[Vec<u32>]) -> u32 {
    let mut n_double_claim = 0;
    for row in 0..fabric.len() {
        for col in 0..fabric[0].len() {
            n_double_claim += (fabric[row][col] > 1) as u32
        }
    }
    n_double_claim
}

fn part2(fabric: &[Vec<u32>], claims: &[Claim]) -> u32 {
    let unique = |claim: &Claim, fabric: &[Vec<u32>]| {
        for row in claim.y .. (claim.y + claim.dy) {
            for col in claim.x .. (claim.x + claim.dx) {    
                if fabric[row as usize][col as usize] > 1 {
                    return false
                }
            }
        }
        return true
    };
    for claim in claims {
        if unique(claim, fabric) { return claim.id }
    }
    panic!("No claim is unique");
}

fn lay_claims(fabric: &mut [Vec<u32>], claims: &[Claim]) {
    for claim in claims {
        for row in claim.y .. (claim.y + claim.dy) {
            for col in claim.x .. (claim.x + claim.dx) {
                fabric[row as usize][col as usize] += 1
            }    
        }
    }
}

fn parse_claim(line: &str) -> Option<Claim> {
    let caps = RE.captures(line).unwrap();
    let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let x = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let y = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
    let dx = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
    let dy = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();
    Some(Claim{id, x, y, dx, dy})
}

fn parse_claims(path: &str) -> Vec<Claim> {
    BufReader::new(File::open(path).expect("Failed to open file")).lines()
    .map(|s| s.expect("Failed to read line")).map(|s| {
        parse_claim(&s).expect(&format!("Failed to parse claim {}", s))
    }).collect()
    
}

fn make_fabric(sidelen: usize) -> Vec<Vec<u32>> {
    let mut res = Vec::new();
    for _i in 0..sidelen {
        res.push(vec![0; sidelen])
    }
    res
}
