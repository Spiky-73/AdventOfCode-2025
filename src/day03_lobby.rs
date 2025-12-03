use std::io::BufRead;

pub fn solve_p1<F: BufRead>(input: F) {
    let mut joltage = 0;

    for l in input.lines() {
        let line = l.unwrap();
        let mut left = 0;
        let mut right = 0;
        let mut chars = line.chars();
        for i in 0..line.len() {
            let c = chars.next().unwrap();
            let v: i32 = c.to_digit(10).unwrap().try_into().unwrap();
            if i != line.len() - 1 && v > left {
                left = v;
                right = 0;
            } else if v > right {
                right = v;
            }
        }
        joltage += left * 10 + right;
    }

    println!("{}", joltage);
}

pub fn solve_p2<F: BufRead>(input: F) {
    const NUM_BATTERIES: usize = 12;
    let mut joltage: u64 = 0;

    for l in input.lines() {
        let line = l.unwrap();
        let mut batteries: [u32; NUM_BATTERIES] = [0; NUM_BATTERIES];
        let mut chars = line.chars();
        for i in 0..line.len() {
            let c = chars.next().unwrap();
            let v = c.to_digit(10).unwrap();
            let min_bat: usize = if i + NUM_BATTERIES < line.len() {
                0
            } else {
                i + NUM_BATTERIES - line.len()
            };
            for b in min_bat..NUM_BATTERIES {
                if v > batteries[b] {
                    batteries[b] = v;
                    for c in b + 1..NUM_BATTERIES {
                        batteries[c] = 0;
                    }
                    break;
                }
            }
            // println!("{:?}", batteries);
        }
        let mut bank: u64 = 0;
        for bat in batteries {
            bank *= 10;
            bank += bat as u64;
        }
        // println!("{}", bank);
        joltage += bank;
    }

    println!("{}", joltage);
}
