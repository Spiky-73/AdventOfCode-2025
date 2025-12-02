use std::io::BufRead;

pub fn solve_p1<F: BufRead>(input: F) {
    let line = input.lines().nth(0).unwrap().unwrap();

    let mut count = 0;

    for range in line.split(',') {
        let nums = range.split('-').collect::<Vec<&str>>();
        let start = nums.iter().nth(0).unwrap().parse::<u64>().unwrap();
        let end = nums.iter().nth(1).unwrap().parse::<u64>().unwrap();
        'outer: for n in start..(end+1) {
            let num = n.to_string();
            if num.len() % 2 != 0 {
                continue;
            };
            for i in 0..(num.len() / 2) {
                if num.chars().nth(i) != num.chars().nth(num.len() / 2 + i) {
                    continue 'outer;
                }
            }
            count += n;
        }
    }

    println!("{}", count);
}

pub fn solve_p2<F: BufRead>(input: F) {
    let line = input.lines().nth(0).unwrap().unwrap();

    let mut count = 0;

    for range in line.split(',') {
        let nums = range.split('-').collect::<Vec<&str>>();
        let start = nums.iter().nth(0).unwrap().parse::<u64>().unwrap();
        let end = nums.iter().nth(1).unwrap().parse::<u64>().unwrap();
        'num: for n in start..(end+1) {
            let num = n.to_string();

            'pat: for p in (1..num.len()/2+1).rev() { // test every pattern length
                if num.len() % p != 0 {
                    continue;
                };
                for i in 0..p { // test every char of the pattern
                    let char = num.chars().nth(i);
                    for j in 1..num.len()/p { // compare it to the char of others patterns
                        if num.chars().nth(i + j*p) != char {
                            continue 'pat; // pattern does not mach, try the next one
                            
                        }
                    }
                }
                // println!("{}", n);
                count += n;
                continue 'num; // we found a match
            }
        }
    }

    println!("{}", count);
}
