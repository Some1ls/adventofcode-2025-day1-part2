use std::fs;

fn main() {
    let input = fs::read_to_string("l/input.txt").unwrap();
    let lines = input.lines();
    let mut position = 50;
    let mut password = 0;
    for line in lines {
        let direction = line.bytes().next().unwrap();
        let steps = line[1..].parse::<i32>().unwrap();
        let displacement = position + if direction == b'L' { -steps } else { steps };

        let rem = displacement.rem_euclid(100);
        password += displacement.div_euclid(100).abs();

        if displacement < 0 {
            if position == 0 {
                password -= 1;
            } else if rem == 0 {
                password += 1;
            }
        } else if displacement == 0 && position != 0 {
            password += 1;
        }
        position = rem;
    }
    println!("{}", password);
}
