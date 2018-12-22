fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let result = get_square(16);
    println!("{:?}", result);
}

fn part_two() {
    let mut max = (0, 0, (0, 0));
    //really slow !!
    for i in 1..20 {
        println!("{}", i);
        let (size, max_power_level, max_power_level_coord) = get_square(i);
        if max_power_level > max.1 {
            max = (size, max_power_level, max_power_level_coord);
        }
    }
    println!("{:?}", max);
}

fn get_square(size: i32) -> (i32, i32, (i32, i32)) {
    let input = 9110;
    let mut max_power_level = 0;
    let mut max_power_level_coord = (0, 0);
    for x in 1..300 - size {
        for y in 1..300 - size {
            let mut power_level = 0;
            for a in x..x + size {
                for b in y..y + size {
                    power_level = power_level + get_cell_power_level((a, b), input);
                }
            }
            if power_level > max_power_level {
                max_power_level = power_level;
                max_power_level_coord = (x, y);
            }
        }
    }
    (size, max_power_level, max_power_level_coord)
}

fn get_cell_power_level((x, y): (i32, i32), serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let power_level_start = rack_id * y;
    let add_serial_number = power_level_start + serial_number;
    let result = add_serial_number * rack_id;
    let digit = result
        .to_string()
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .nth(2)
        .or(Some(0))
        .unwrap();
    digit as i32 - 5
}
