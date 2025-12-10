use std::{collections::HashSet, io::BufRead};

pub fn solve_p1<F: BufRead>(input: F) {
    let red_tiles: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut max_area = 0;

    for i in 1..red_tiles.len() {
        for j in 0..i {
            let area = (red_tiles[i][0].abs_diff(red_tiles[j][0]) + 1)
                * (red_tiles[i][1].abs_diff(red_tiles[j][1]) + 1);
            if area > max_area {
                max_area = area;
                println!(
                    "[({}, {}), ({}, {})]: {}",
                    red_tiles[i][0], red_tiles[i][1], red_tiles[j][0], red_tiles[j][1], area
                )
            }
        }
    }
    println!("{}", max_area);
}

pub fn solve_p2<F: BufRead>(input: F) {
    let red_tiles: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut green_tiles: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..red_tiles.len() {
        let delta_x = red_tiles[i][0].abs_diff(red_tiles[(i + 1) % red_tiles.len()][0]);
        let delta_y = red_tiles[i][1].abs_diff(red_tiles[(i + 1) % red_tiles.len()][1]);
        let start_x = red_tiles[i][0].min(red_tiles[(i + 1) % red_tiles.len()][0]);
        let start_y = red_tiles[i][1].min(red_tiles[(i + 1) % red_tiles.len()][1]);
        for x in start_x..(start_x + delta_x + 1) {
            for y in start_y..(start_y + delta_y + 1) {
                green_tiles.insert((x, y));
            }
        }
    }

    let mut max_area = 0;

    for i in 1..red_tiles.len() {
        'red_tile: for j in 0..i {
            let delta_x = red_tiles[i][0].abs_diff(red_tiles[j][0]);
            let delta_y = red_tiles[i][1].abs_diff(red_tiles[j][1]);

            let area = (delta_x + 1) * (delta_y + 1);
            if area <= max_area {
                continue 'red_tile;
            }
            let start_x = red_tiles[i][0].min(red_tiles[j][0]);
            let start_y = red_tiles[i][1].min(red_tiles[j][1]);

            for y in (start_y+1)..(start_y + delta_y) {
                for x in (start_x+1)..(start_x + delta_x) {
                    if green_tiles.contains(&(x, y)) {
                        continue 'red_tile;
                    }
                }
            }
            max_area = area;
            println!(
                "[({}, {}), ({}, {})]: {}",
                red_tiles[i][0], red_tiles[i][1], red_tiles[j][0], red_tiles[j][1], area
            )
        }
    }
    println!("{}", max_area);
}
