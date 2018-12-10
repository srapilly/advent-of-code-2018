#[derive(Debug)]
struct Node {
    childs: Vec<Node>,
    metadatas: Vec<usize>,
}

fn main() {
    let input = include_str!("input.txt");
    let node = parse_node(&mut parse(input));
    let result_1 = star_one(&node);
    println!("Day 8 - Part 1 : {}", result_1);
    let result_2 = star_two(&node);
    println!("Day 8 - Part 2 : {}", result_2);
}

fn star_one(node: &Node) -> usize {
    let mut sum = 0;
    for child in &node.childs {
        sum += star_one(child);
    }
    let meta_sum: usize = node.metadatas.iter().sum();
    meta_sum + sum
}

fn star_two(node: &Node) -> usize {
    if node.childs.is_empty() {
        let sum = node.metadatas.iter().sum();
        //println!("no childs : sum -> {}", sum);
        sum
    } else {
        let mut sum = 0;
        for index in &node.metadatas {
            sum += node
                .childs
                .get(*index - 1)
                .and_then(|child| Some(star_two(child)))
                .unwrap_or(0)
        }
        sum
    }
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .flat_map(|c| c.parse::<usize>().ok())
        .collect()
}

fn parse_node(data: &mut Vec<usize>) -> Node {
    let nb_child = data.remove(0);
    let nb_metadata = data.remove(0);

    let mut childs = vec![];
    for _ in 0..nb_child {
        childs.push(parse_node(data));
    }

    let mut metadatas = vec![];
    for _ in 0..nb_metadata {
        metadatas.push(data.remove(0));
    }

    Node { childs, metadatas }
}
