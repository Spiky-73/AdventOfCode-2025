use std::io::BufRead;

pub fn solve_p1<F: BufRead>(input: F) {
    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut splits = 0;

    println!("{}", map[0].iter().collect::<String>());
    for y in 0..map.len() - 1 {
        for x in 0..map[y].len() {
            if map[y][x] != 'S' && map[y][x] != '|' {
                continue;
            }
            if map[y + 1][x] != '^' {
                map[y + 1][x] = '|';
            } else {
                map[y + 1][x - 1] = '|';
                map[y + 1][x + 1] = '|';
                splits += 1
            }
        }
        println!("{}", map[y + 1].iter().collect::<String>());
    }

    println!("{}", splits);
}

pub fn solve_p2<F: BufRead>(input: F) {
    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut timelines: Vec<Vec<usize>> =
        map.iter().map(|l| l.iter().map(|_| 0).collect()).collect();

    println!(
        "{} - {}",
        map[0].iter().collect::<String>(),
        timelines[0].iter().sum::<usize>()
    );
    for y in 0..map.len() - 1 {
        for x in 0..map[y].len() {
            if map[y][x] != 'S' && map[y][x] != '|' {
                continue;
            }
            let tl = if map[y][x] == 'S' {
                1
            } else {
                timelines[y][x]
            };
            if map[y + 1][x] != '^' {
                map[y + 1][x] = '|';
                timelines[y + 1][x] += tl;
            } else {
                map[y + 1][x - 1] = '|';
                map[y + 1][x + 1] = '|';
                timelines[y + 1][x - 1] += tl;
                timelines[y + 1][x + 1] += tl;
            }
        }
        println!(
            "{} - {}",
            map[y + 1].iter().collect::<String>(),
            timelines[y + 1].iter().sum::<usize>()
        );
    }

    println!("{}", timelines[map.len() - 1].iter().sum::<usize>());
}
