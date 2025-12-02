use std::fs;

fn main() {
    let input = fs::read_to_string("l/input2.txt").unwrap();
    let lines = input.lines();
    let mut previous_pos: i32 = 0;
    let mut pos = 50;
    let mut total_hits = 0i32;
    // let mut password = 0;
    for line in lines {
        let direction = line.bytes().next().unwrap();
        let steps = line[1..].parse::<i32>().unwrap();
        // previous_pos = pos;
        if direction == b'L' {
            let asa = pos - steps;
            let a = asa.div_euclid(100).abs();
            if (a != 0 || pos == steps) && pos != 0 {
                total_hits += 1;
            }
            pos = asa.rem_euclid(100);
        } else {
            let asa = pos + steps;
            let a = asa.div_euclid(100).abs();
            if (a != 0 || pos == steps) && pos != 0 {
                total_hits += 1;
            }
            pos = asa.rem_euclid(100);
        }
        // expect 1 1 2 2 3 4 4 5 6
        print!("{} ", total_hits);
    }
    println!("");
    // println!("total rotations: {}", total_hits);
}
