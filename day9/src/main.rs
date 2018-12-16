use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    star_one();
}

fn star_one() {
    let nb_players = 405;
    let last_marble = 71700 * 100;

    let mut circle = VecDeque::new();
    circle.push_back(0);
    circle.push_back(4);
    circle.push_back(2);
    circle.push_back(1);
    circle.push_back(3);

    let mut index_current = 1;
    let mut current_player = 5;
    let mut players_score: HashMap<usize, usize> = HashMap::new();

    for next_marble in 5..last_marble + 1 {
        //println!("marble: {}", next_marble);
        //result :::    3628143500)
        if next_marble % 23 == 0 {
            //removed index - 7
            if index_current < 7 {
                index_current = circle.len() - (7 - index_current);
            } else {
                index_current = index_current - 7;
            }
            let marble_removed = circle.remove(index_current).unwrap();
            //println!("removed: {}, new index: {}", marble_removed, index_current);
            //keep current marble
            players_score
                .entry(current_player)
                .and_modify(|score| *score += next_marble + marble_removed)
                .or_insert(next_marble + marble_removed);
        } else {
            index_current = if index_current != circle.len() - 1 {
                index_current + 2
            } else {
                1
            };
            circle.insert(index_current, next_marble);
            //println!("{:?}", circle);
        }

        current_player = if current_player == nb_players {
            1
        } else {
            current_player + 1
        }
    }

    println!("{:?}", players_score);
    let result = players_score.values().cloned().max();
    println!("{:?}", result);
}
