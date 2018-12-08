use std::num::ParseIntError;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug,  Hash, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}


impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let vec: Vec<&str> = s.split(',').map(|s| s.trim()).collect();

        let x = vec[0].parse::<i32>()?;
        let y = vec[1].parse::<i32>()?;
        Ok(Point { x, y })
    }
}

fn main() {
    distance(&Point { x: 10, y: 10 }, &Point { x: 20, y: 20 });
    let input = include_str!("input.txt");
    star_one(input);
    star_two(input);
}

fn star_one(input: &str) {
    let list: Vec<Point> = input
        .lines()
        .map(|line| line.parse::<Point>().unwrap())
        .collect();

    let mut grid: Vec<Vec<Option<Point>>> = Vec::with_capacity(400);
    for x in 0..400 {
        grid.insert(x, Vec::with_capacity(400));
        for y in 0..400 {
            let p1 = Point { x: x as i32, y };
            let min_point = list.iter().min_by_key(|&p2| distance(&p1, p2)).unwrap();
            let min_distance = distance(&p1, min_point);
            if list.iter().filter(|&p2| min_distance == distance(&p1, p2)).count() == 1 {
                grid.get_mut(x).unwrap().insert(y as usize, Some(*min_point));
            }
            else {
                grid.get_mut(x).unwrap().insert(y as usize, None);
            }
        }
    }

    let mut map: HashMap<Point, Option<usize>> =  list.iter().map(|p| (*p, Some(0))).collect();


    for (x, vec) in grid.iter().enumerate() {
        for (y , cell) in vec.iter().enumerate() {
            if cell.is_some() {
                let point = cell.unwrap();
                if x == 0 || x == 399 || y == 0 || y == 399 {
                    map.entry(point).and_modify(|count| { *count = count.and_then(|_| None)});
                }
                else {
                    map.entry(point).and_modify(|count| { *count = count.and_then(|val| Some(val+1))});
                }
            }

        }
    }
    println!("{:?}", map);
}

fn star_two(input: &str) {
    let list: Vec<Point> = input
        .lines()
        .map(|line| line.parse::<Point>().unwrap())
        .collect();

    let mut counter = 0;
    for x in 0..400 {
        for y in 0..400 {
            let p = Point { x, y };
            let total_distance: i32 = list.iter().map(|safe_point| distance(&p, safe_point)).sum();
            if total_distance < 10000 { counter+=1 }
        }
    }
    println!("Day 6 - Part 2 : {}", counter);
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}