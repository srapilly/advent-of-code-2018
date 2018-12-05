use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    star_one(input);
    star_two(input);
}

fn star_one(input: &str) {
    let mut deque: VecDeque<char> = VecDeque::new();
    for c in input.chars() {
        update_deque(&mut deque, c)
    }
    println!("Day 5 - Part 1 : {}", deque.len());
}

fn update_deque(deque: &mut VecDeque<char>, c: char) -> () {
    let reacting = deque.back()
        .map_or(false, |x| is_react(*x, c));
    if reacting {
        deque.pop_back();
    } else {
        deque.push_back(c);
    }
}

fn is_react(x: char, y: char) -> bool {
    return x.eq_ignore_ascii_case(&y)
        && (x.is_ascii_uppercase() && y.is_ascii_lowercase()
        || y.is_ascii_uppercase() && x.is_ascii_lowercase());
}

fn star_two(input: &str) {
    let alphabet = ['a', 'b', 'c', 'd', 'e', 'f',
        'g', 'h', 'i', 'j', 'k', 'l',
        'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x',
        'y', 'z'];

    let result = alphabet.iter()
        .map(|char_to_remove| {
            let mut deque:  VecDeque<char> = VecDeque::new();
            for c in input.chars() {
                if !c.eq_ignore_ascii_case(char_to_remove) {
                    update_deque(&mut deque, c)
                }
            }
            deque.len()
        })
        .min();
    println!("Day 5 - Part 2 : {}", result.unwrap());
}