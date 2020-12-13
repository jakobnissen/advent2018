use std::collections::HashSet;

fn main() {
    let numbers = parse_numbers("input.txt");
    println!("Solution to day 1 part 1: {}", numbers.iter().sum::<i64>());
    println!("Solution to day 1 part 2: {}", part2(&numbers));
}

fn parse_numbers(path: &str) -> Vec<i64> {
    std::fs::read_to_string(path).expect("Failed to read file").trim().lines()
    .map(|s| s.parse::<i64>().expect(&format!("Failed to parse string {}", s))).collect()
}

fn part2(numbers: &[i64]) -> i64 {
    let mut freq: i64 = 0;
    let mut set: HashSet<i64> = HashSet::new();
    set.insert(freq.clone());
    loop {
        for num in numbers {
            freq += num;
            if !set.insert(freq) {
                return freq
            }
        }
    }
}
