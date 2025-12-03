use std::fs;

fn main() {
    let input = fs::read_to_string("l/input.txt").unwrap();
    let lines = input.lines();
    let mut pos = 50;
    let mut password = 0;
    for line in lines {
        let direction = line.bytes().next().unwrap();
        let steps = line[1..].parse::<i32>().unwrap();
        if direction == b'L' {
            let displacement = pos - steps;
            let mut hits = displacement.div_euclid(100).abs();
            if displacement == 0 {
                if pos != 0 {
                    password += 1;
                }
            } else if displacement < 0 {
                if pos == 0 {
                    hits -= 1;
                }
                if displacement % 100 == 0 {
                    password += 1;
                }
                password += hits;
            } else {
                password += hits;
            }
            pos = displacement.rem_euclid(100);
        } else {
            let displacement = pos + steps;
            let hits = displacement.div_euclid(100).abs();
            password += hits;
            pos = displacement.rem_euclid(100);
        }
        println!("{}", password);
    }
}
