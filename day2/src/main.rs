use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    star_one(&input);
    star_two(&input);
}

fn star_one(input: &str) {
    let (count_two, count_three) = input
        .lines()
        .map(|line| count(line))
        .fold((0,0), |acc, val| (acc.0 + val.0, acc.1 + val.1));

    let result = count_two * count_three;
    println!("Day 2 - Part 1 : {}", result)
}

fn count(line: &str) -> (i32, i32) {
    let mut map: HashMap<char, u64> = HashMap::new();
    line.chars().for_each(|c| {
        map.entry(c)
            .and_modify(|count| { *count += 1 })
            .or_insert(1);
    });
    let two = map.iter().any(|(_,&val)| val == 2 );
    let three = map.iter().any(|(_,&val)| val == 3 );
    (two as i32, three as i32)
}

fn star_two(input: &str) {
    for  (i,line) in input.lines().enumerate()  {
        for line2 in input.lines().skip(i + 1) {
            let result = differ_by_one(line, line2);
            if result.is_some() {
                println!("Day 2 - Part 2 : {}", result.unwrap());
                break
            }
        }
    }
}

fn differ_by_one(x: &str, y: &str) -> Option<String> {
    let size = x.len();
    let differing_result: String = x.chars().zip(y.chars())
        .filter(|(val1,val2)| val1 == val2)
        .map(|(val,_)| val)
        .collect();
    if differing_result.len() == size - 1 {
        return Some(differing_result);
    }
    None
}