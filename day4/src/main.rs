extern crate chrono;

use chrono::DateTime;
use ::Action::Sleep;
use ::Action::Wake;
use ::Action::Begin;
use chrono::Utc;
use chrono::offset::TimeZone;
use std::collections::HashMap;
use chrono::Timelike;

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
struct Event {
    date: DateTime<Utc>,
    action: Action,
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
enum Action {
    Sleep,
    Wake,
    Begin(isize)
}

fn main() {
    let input = include_str!("input.txt");
    let mut vec: Vec<Event> = input.lines().map(parse_event).collect();
    vec.sort();

    let mut total_minutes: HashMap<isize, isize> = HashMap::new();
    let mut minutes: HashMap<isize, HashMap<isize, isize>>  = HashMap::new();

    let mut current_id = 0 ;
    let mut date_asleep = Utc::now();
    for event in vec {
        match event.action {
            Begin(id) => {
                current_id = id;
                minutes.entry(current_id).or_insert(HashMap::new());
            },
            Sleep => date_asleep = event.date,
            Wake => {
                let time_asleep = (event.date - date_asleep).num_minutes() as isize;
                total_minutes.entry(current_id).and_modify(|time| *time += time_asleep).or_insert(time_asleep);
                for m in date_asleep.minute()..event.date.minute() {
                    minutes.entry(current_id).and_modify(|map| {
                        map.entry(m as isize).and_modify(|x| *x += 1).or_insert(1);
                    });
                }
            }
        }
    }
    let result_id = total_minutes.iter().max_by_key(|(_,val)| *val).unwrap().0;
    let result_minute = minutes.get(result_id).unwrap().iter().max_by_key(|(_,val)| *val).unwrap().0;
    println!("Day 4 - Part 1 : {:?}", result_id * result_minute);

    let result = minutes.iter().fold((0,0,0), |(global_id,global_max,global_minute), map| {
        let max_value = map.1.iter().max_by_key(|(_,val)| *val).unwrap_or_else(|| (&0,&0));
        if *max_value.1 > global_max {
            return (*map.0, *max_value.1, *max_value.0);
        }
        return (global_id, global_max, global_minute);
    });
    println!("Day 4 - Part 2 : {:?}", result.0 * result.2);
}

fn parse_event(line: &str) -> Event  {
    let date = Utc.datetime_from_str(&line[1..=16], "%Y-%m-%d %H:%M").unwrap();
    let action = match line.chars().nth(19).unwrap() {
        'f' => Sleep,
        'w' => Wake,
        'G' => {
            let number = (&line[26..=29])
                .split_whitespace().nth(0).unwrap()
                .parse::<isize>().unwrap();
            Begin(number)
        }
        _ => panic!("parse error")
    };
    Event {date, action}
}