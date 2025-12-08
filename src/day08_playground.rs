use std::{collections::HashSet, io::BufRead};

pub fn solve_p1<F: BufRead>(input: F) {
    let junctions: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let mut circuits = 0;
    let mut junction_circuit: Vec<usize> = junctions.iter().map(|_| 0).collect();

    let mut sorted_distances: Vec<(usize, usize, f32)> = Vec::new();
    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            let d = (((junctions[j][0] as f32 - junctions[i][0] as f32).powf(2.)
                + (junctions[j][1] as f32 - junctions[i][1] as f32).powf(2.)
                + (junctions[j][2] as f32 - junctions[i][2] as f32).powf(2.))
                as f32)
                .powf(0.5);
            sorted_distances.push((i, j, d));
        }
    }

    sorted_distances.sort_by(|&(_, _, d1), &(_, _, d2)| d1.total_cmp(&d2));

    for s in 0..1000 {
        let pair = (sorted_distances[s].0, sorted_distances[s].1);

        print!("{}: ", s);
        if junction_circuit[pair.0] == 0 && junction_circuit[pair.1] == 0 {
            println!(
                "{:?}, {:?} - ++{}",
                junctions[pair.0],
                junctions[pair.1],
                circuits + 1
            );
            circuits += 1;
            junction_circuit[pair.0] = circuits;
            junction_circuit[pair.1] = circuits;
        } else if junction_circuit[pair.0] == 0 {
            println!(
                "{:?}, {:?} - >>{}",
                junctions[pair.0], junctions[pair.1], junction_circuit[pair.1]
            );
            junction_circuit[pair.0] = junction_circuit[pair.1];
        } else if junction_circuit[pair.1] == 0 {
            println!(
                "{:?}, {:?} - {}<<",
                junctions[pair.0], junctions[pair.1], junction_circuit[pair.0]
            );
            junction_circuit[pair.1] = junction_circuit[pair.0];
        } else {
            println!(
                "{:?}, {:?} - {}={}",
                junctions[pair.0],
                junctions[pair.1],
                junction_circuit[pair.0],
                junction_circuit[pair.1]
            );
            let c = junction_circuit[pair.1];
            for j in 0..junctions.len() {
                if junction_circuit[j] == c {
                    junction_circuit[j] = junction_circuit[pair.0];
                }
            }
        }
    }

    let mut counts: Vec<usize> = (0..circuits - 1)
        .map(|c| junction_circuit.iter().filter(|v| **v == c + 1).count())
        .collect();

    println!("{:?}", counts);
    counts.sort();

    let value = counts[counts.len() - 1] * counts[counts.len() - 2] * counts[counts.len() - 3];
    println!("{}", value);
}

pub fn solve_p2<F: BufRead>(input: F) {
    let junctions: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut circuits = 0;
    let mut junction_circuit: Vec<usize> = junctions.iter().map(|_| 0).collect();

    let mut dead_circuits = 0;
    let mut unconnected_junctions = junctions.len();

    let mut sorted_distances: Vec<(usize, usize, f32)> = Vec::new();
    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            let d = (((junctions[j][0] as f32 - junctions[i][0] as f32).powf(2.)
                + (junctions[j][1] as f32 - junctions[i][1] as f32).powf(2.)
                + (junctions[j][2] as f32 - junctions[i][2] as f32).powf(2.))
                as f32)
                .powf(0.5);
            sorted_distances.push((i, j, d));
        }
    }
    sorted_distances.sort_by(|&(_, _, d1), &(_, _, d2)| d1.total_cmp(&d2));

    let mut s = 0;
    while unconnected_junctions != 0 || circuits - dead_circuits != 1 {
        let pair = (sorted_distances[s].0, sorted_distances[s].1);
        s += 1;

        print!("{}: ", s);
        if junction_circuit[pair.0] == 0 && junction_circuit[pair.1] == 0 {
            circuits += 1;
            junction_circuit[pair.0] = circuits;
            junction_circuit[pair.1] = circuits;
            unconnected_junctions -= 2;
            println!(
                "{:?}, {:?} - ++{}",
                junctions[pair.0], junctions[pair.1], circuits
            );
        } else if junction_circuit[pair.0] == 0 {
            junction_circuit[pair.0] = junction_circuit[pair.1];
            unconnected_junctions -= 1;
            println!(
                "{:?}, {:?} - >>{}",
                junctions[pair.0], junctions[pair.1], junction_circuit[pair.1]
            );
        } else if junction_circuit[pair.1] == 0 {
            unconnected_junctions -= 1;
            junction_circuit[pair.1] = junction_circuit[pair.0];
            println!(
                "{:?}, {:?} - {}<<",
                junctions[pair.0], junctions[pair.1], junction_circuit[pair.0]
            );
        } else if junction_circuit[pair.0] != junction_circuit[pair.1] {
            let c = junction_circuit[pair.1];
            for j in 0..junctions.len() {
                if junction_circuit[j] == c {
                    junction_circuit[j] = junction_circuit[pair.0];
                }
            }
            dead_circuits += 1;
            println!(
                "{:?}, {:?} - {}={}",
                junctions[pair.0], junctions[pair.1], junction_circuit[pair.0], c
            );
        } else {
            println!("{:?}, {:?}", junctions[pair.0], junctions[pair.1],);
        }
    }

    let x_mul = junctions[sorted_distances[s-1].0][0] * junctions[sorted_distances[s-1].1][0];
    println!("{}", x_mul)
}
