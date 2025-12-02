use std::io::BufRead;

pub fn solve_p1<F: BufRead>(input: F) {
    const MODULO: i32 = 100;
    let mut position: i32 = 50;

    let mut value = 0;

    for l in input.lines() {
        let line = l.unwrap();
        let direction = if line.chars().nth(0).unwrap() == 'L' {
            -1
        } else {
            1
        };
        let offset = line.split_at(1).1.parse::<i32>().unwrap();
        position = (position + direction * offset + MODULO) % MODULO;
        if position == 0 {
            value += 1;
        }
        // println!("{} : {} : {}", position, direction, offset);
    }
    
    println!("{}", value);
}

pub fn solve_p2<F: BufRead>(input: F) {
    const MODULO: i32 = 100;
    let mut position: i32 = 50;
    
    let mut value = 0;
    
    for l in input.lines() {
        let line = l.unwrap();
        let direction = if line.chars().nth(0).unwrap() == 'L' {
            -1
        } else {
            1
        };
        let count = line.split_at(1).1.parse::<i32>().unwrap();
        for _ in 0..count {
            position += direction;
            if position == -1 { position += MODULO; }
            else if position == MODULO { position -= MODULO }
            if position == 0 { value += 1; }
        }
    }
    println!("{}", value);
}
