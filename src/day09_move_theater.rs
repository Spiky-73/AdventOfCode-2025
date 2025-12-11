use std::io::BufRead;

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

    let mut lines: Vec<(usize, usize, usize, usize)> = Vec::new();
    for i in 0..red_tiles.len() {
        lines.push((
            red_tiles[i][0].min(red_tiles[(i + 1) % red_tiles.len()][0]),
            red_tiles[i][1].min(red_tiles[(i + 1) % red_tiles.len()][1]),
            red_tiles[i][0].max(red_tiles[(i + 1) % red_tiles.len()][0]),
            red_tiles[i][1].max(red_tiles[(i + 1) % red_tiles.len()][1]),
        ));
    }

    let mut max_area = 0;

    for i in 1..red_tiles.len() {
        'red_tile: for j in 0..i {
            let left = red_tiles[i][0].min(red_tiles[j][0]);
            let top = red_tiles[i][1].min(red_tiles[j][1]);
            let right = red_tiles[i][0].max(red_tiles[j][0]);
            let bot = red_tiles[i][1].max(red_tiles[j][1]);

            let area = (right - left + 1) * (bot - top + 1);
            if area <= max_area {
                continue 'red_tile;
            }

            for &(l, t, r, b) in lines.iter() {
                let inter_w = ((right - 1).min(r) + 1).saturating_sub((left + 1).max(l));
                let inter_h = ((bot - 1).min(b) + 1).saturating_sub((top + 1).max(t));
                if inter_w != 0 && inter_h != 0 {
                    continue 'red_tile;
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
