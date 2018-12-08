use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    star_two(input);
}

#[derive(Debug, Copy, Clone)]
struct Step {
    requirement: char,
    value: char
}

fn star_one(input: &str) {
    let mut requirements: Vec<Step> = input
        .lines()
        .map(parse_line)
        .collect();

    let mut steps: HashSet<char> = (65..=90).map(|x| char::from(x)).collect(); // range A..Z
    let mut result = String::new();

    while !steps.is_empty(){
        let found = find_step(&steps, &requirements);
        steps.remove(&found);
        requirements  = requirements.into_iter().filter(|step| step.requirement != found).collect();
        result.push(found);
    }
    println!("Day 7 - Part 1 {}", result);
}

fn star_two(input : &str) {
    let mut requirements: Vec<Step> = input
        .lines()
        .map(parse_line)
        .collect();

    let mut steps: HashSet<char> = (65..=90).map(|x| char::from(x)).collect(); // range A..Z
    let mut time_step_done: HashMap<i32, char> = HashMap::new();
    let mut time = 0;

    while !steps.is_empty() || !time_step_done.is_empty() {
        println!("Current time : {}", time);
        println!("Steps in progress {:?}", time_step_done);
        match time_step_done.remove(&time) {
            Some(step_finish) => {
                println!("Time: {} Task Done: {}", time, step_finish);
                requirements  = requirements.into_iter().filter(|step| step.requirement != step_finish).collect();
            },
            None => ()
        }

        let found = find_steps(&steps, &requirements);
        let workers_available = 5 - time_step_done.len();
        println!("Steps available : {:?}", found);
        println!("Workers available : {:?}\n", workers_available);
        for x in 0..(found.len()).min(workers_available) {
            steps.remove(&found[x]);
            time_step_done.insert(time + step_duration(found[x]), found[x]);
        }
        time+=1;
    }
    println!("Day 7 - Part 2 : {}", time - 1);

}

fn step_duration(step: char) -> i32 {
    60 + (step as i32 - 64)
}


fn find_steps(steps: &HashSet<char>, requirements: &Vec<Step>) -> Vec<char> {
    let mut result: Vec<char> = steps.iter().cloned()
        .filter(|&c| is_available(c, &requirements))
        .collect();
    result.sort();
    result
}

fn find_step(steps: &HashSet<char>, requirements: &Vec<Step>) -> char {
    let result = find_steps(steps, requirements);
    result[0]
}

fn is_available(c: char, requirements: &Vec<Step>) -> bool {
    requirements
        .iter()
        .all(|step| step.value != c)
}

fn parse_line(line: &str) -> Step {
    let step: Vec<char> = line
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .skip(1)
        .collect();
    Step { requirement: step[0], value: step[1] }
}