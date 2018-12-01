use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    star_one(&input);
    star_two(&input);
}

fn star_one(input: &str) {
    let result = input
        .lines()
        .fold(0, |acc,line|  acc + line.parse::<isize>().unwrap());
    println!("Day 1 - Part 1 : {}", result)
}

fn star_two(input: &str) {
    let mut set = HashSet::new();
    let result = input
        .lines()
        .map(|s| s.parse::<isize>().unwrap())
        .cycle()
        .try_fold(0, |acc,number| {
            let frequency = acc + number;
            match set.insert(frequency) {
                true => Ok(frequency),
                false => Err(frequency)
            }
        } ).unwrap_err();
    println!("Day 1 - Part 2 : {}", result)
}