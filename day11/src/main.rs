fn main() {
    let input = 9110;
    let mut max_power_level = 0;
    let mut max_power_level_coord = (0, 0);
    for x in 1..300 - 3 {
        for y in 1..300 - 3 {
            let mut power_level = 0;
            for a in x..x + 3 {
                for b in y..y + 3 {
                    power_level = power_level + get_cell_power_level((a, b), input);
                }
            }
            if power_level > max_power_level {
                max_power_level = power_level;
                max_power_level_coord = (x, y);
            }
        }
    }
    println!("{} {:?}", max_power_level, max_power_level_coord)
}

fn get_cell_power_level((x, y): (usize, usize), serial_number: usize) -> i32 {
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
