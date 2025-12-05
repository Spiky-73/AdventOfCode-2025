use std::io::BufRead;

pub fn solve_p1<F: BufRead>(input: F) {
    let mut count = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();

    for l in input.lines() {
        grid.push(l.unwrap().chars().collect());
    }

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let mut c = 0;
            if grid[y][x] != '@' {
                print!(".");
                continue;
            }
            for dy in -1..2 {
                for dx in -1..2 {
                    let i = (x as isize) + dx;
                    let j = (y as isize) + dy;
                    if 0 <= i && i < (grid[y].len() as isize) && 0 <= j && j < (grid.len() as isize)
                    {
                        if grid[j as usize][i as usize] == '@' {
                            c += 1;
                        }
                    }
                }
            }
            if c < (4 + 1) {
                print!("x");
                count += 1
            } else {
                print!("@");
            }
        }
        println!("");
    }

    println!("{}", count);
}

pub fn solve_p2<F: BufRead>(input: F) {
    let mut removed = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();

    for l in input.lines() {
        grid.push(l.unwrap().chars().collect());
    }

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let mut c = 0;
                if grid[y][x] != '@' {
                    print!(".");
                    continue;
                }
                for dy in -1..2 {
                    for dx in -1..2 {
                        let i = (x as isize) + dx;
                        let j = (y as isize) + dy;
                        if 0 <= i
                            && i < (grid[y].len() as isize)
                            && 0 <= j
                            && j < (grid.len() as isize)
                        {
                            if grid[j as usize][i as usize] == '@' {
                                c += 1;
                            }
                        }
                    }
                }
                if c < (4 + 1) {
                    print!("x");
                    to_remove.push((x, y));
                } else {
                    print!("@");
                }
            }
            println!("");
        }
        println!("{}", to_remove.len());
        if to_remove.len() == 0 {
            break;
        }
        removed += to_remove.len();
        for (x, y) in to_remove {
            grid[y][x] = '.';
        }
    }
    println!("{}", removed);
}
