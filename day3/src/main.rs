use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Claim {
    id: usize,
    left_offset: usize,
    top_offset: usize,
    width: usize,
    height: usize
}

fn main() {
    let input = include_str!("input.txt");
    star_one_two(input);
}

fn star_one_two(input: &str) {
    let mut map = HashMap::new();
    input
        .lines()
        .map(|line| line_to_struct(line))
        .for_each(|claim| add(&mut map, claim));
    let result = map.values().filter(|&count| *count > 1).count();
    println!("Day 3 - Part 1 : {}", result);

    let result2 = input
        .lines()
        .map(|line| line_to_struct(line))
        .find(|claim| check_claim(&map, *claim)).unwrap();
    println!("Day 3 - Part 2 : {:?}", result2);

}

fn check_claim(map: &HashMap<(usize,usize), usize>, claim: Claim) -> bool {
    for i in claim.left_offset..claim.left_offset+claim.width {
        for j in claim.top_offset..claim.top_offset+claim.height {
            if *map.get(&(i,j)).unwrap() > 1 {
                return false;
            }
        }
    }
    return true;
}

fn add(map: &mut HashMap<(usize,usize), usize>, claim: Claim) {
    for i in claim.left_offset..claim.left_offset+claim.width {
        for j in claim.top_offset..claim.top_offset+claim.height {
            map.entry((i,j)).and_modify(|count| *count+=1).or_insert(1);
        }
    }
}

fn line_to_struct(line: &str) -> Claim {
    //#8 @ 972,677: 14x17
    let end_id = line.find(char::is_whitespace).unwrap();
    let end_left_offset = line.find(',').unwrap();
    let end_top_offset = line.find(':').unwrap();
    let end_width = line.find('x').unwrap();

    let id = (&line[1..end_id]).parse::<usize>().unwrap();
    let left_offset = (&line[end_id+3..end_left_offset]).parse::<usize>().unwrap();
    let top_offset = (&line[end_left_offset+1..end_top_offset]).parse::<usize>().unwrap();
    let width = (&line[end_top_offset+2..end_width]).parse::<usize>().unwrap();
    let height = (&line[end_width+1..]).parse::<usize>().unwrap();

    return Claim {id, left_offset, top_offset, width, height }
}