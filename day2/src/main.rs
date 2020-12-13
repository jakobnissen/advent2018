use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let strings = read_strings("input.txt");
    println!("Solution to day 2 part 1: {}", part1(&strings));
    println!("Solution to day 2 part 2: {}", part2(&strings));
}

fn part1(strings: &[String]) -> u32 {
    let mut map: HashMap<char, u32> = HashMap::new();
    let (mut two, mut three): (u32, u32) = (0, 0);
    for line in strings {
        let (tw, th) = two_three(&mut map, line);
        two += tw as u32;
        three += th as u32;
    }
    two * three
}

fn two_three(map: &mut HashMap<char, u32>, string: &str) -> (bool, bool) {
    map.clear();
    for char in string.chars() {
        let n = match map.get(&char) {
            Some(n) => *n,
            None => 0
        };
        map.insert(char, n + 1);
    }
    let two = map.values().any(|n| *n == 2);
    let three = map.values().any(|n| *n == 3);
    return (two, three)
}

fn part2(strings: &[String]) -> String {
    for i in 0..strings.len() - 1 {
        let stringi = &strings[i];
        for j in i+1 .. strings.len() {
            let stringj = &strings[j];
            let diff = string_diff(&stringi, &stringj);
            if diff == 1 {
                let mut result = String::new();
                for (i, j) in stringi.chars().zip(stringj.chars()) {
                    if i == j {
                        result.push(i)
                    }
                }
                return result
            }
            
        }
    }
    panic!("No similar strings")
}

fn read_strings(path: &str) -> Vec<String> {
    BufReader::new(File::open(path).expect("Failed to open path")).lines()
    .map(|s| s.unwrap()).filter(|s| !s.trim().is_empty()).collect()
}

fn string_diff(str1: &str, str2: &str) -> u32 {
    str1.chars().zip(str2.chars()).map(|(c1, c2)| (c1 != c2) as u32).sum()
}
