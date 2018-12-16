#[derive(Debug)]
struct Point {
    pos_x: isize,
    pos_y: isize,
    v_x: isize,
    v_y: isize,
}

fn main() {
    let input = include_str!("input.txt");

    let mut data: Vec<Point> = input.lines().map(parse_line).collect();

    for bid in 0..10137 {
        data = data
            .iter()
            .map(|p| Point {
                pos_x: p.pos_x + p.v_x,
                pos_y: p.pos_y + p.v_y,
                v_x: p.v_x,
                v_y: p.v_y,
            })
            .collect();
        //Only show when the point are close to each others
        if bid >= 10120 {
            for y in 125..140 {
                for x in 150..240 {
                    let found = data.iter().find(|p| p.pos_x == x && p.pos_y == y);
                    if found.is_some() {
                        print!("#")
                    } else {
                        print!("-");
                    }
                }
                println!();
            }
            println!();
        }
    }
}

fn parse_line(line: &str) -> Point {
    let pos_x = (&line[11..16]).trim().parse::<isize>().unwrap();
    let pos_y = (&line[18..24]).trim().parse::<isize>().unwrap();

    let v_x = (&line[36..38]).trim().parse::<isize>().unwrap();
    let v_y = (&line[40..42]).trim().parse::<isize>().unwrap();
    Point {
        pos_x,
        pos_y,
        v_x,
        v_y,
    }
}
