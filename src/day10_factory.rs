use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
};

pub fn solve_p1<F: BufRead>(input: F) {
    let mut total_presses = 0;
    for line in input.lines().map(|l| l.unwrap()) {
        let parts: Vec<&str> = line.split(' ').collect();
        let all_indicators: Vec<char> = parts[0][1..parts[0].len() - 1].chars().collect();
        let all_buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1]
            .iter()
            .map(|b| {
                b[1..b.len() - 1]
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .collect();

        print!("[{}]: ", all_indicators.iter().collect::<String>());
        let mut states: VecDeque<(Vec<char>, Vec<usize>)> = VecDeque::new();
        states.push_back((
            all_indicators.iter().map(|_| '.').collect(),
            (0..all_buttons.len()).collect(),
        ));

        while states.len() > 0 {
            let (indicators, buttons) = states.pop_front().unwrap();
            if indicators == all_indicators {
                total_presses += all_buttons.len() - buttons.len();
                println!("{}", all_buttons.len() - buttons.len());
                break;
            }
            for i in 0..buttons.len() {
                let mut b_indicators = indicators.clone();
                for &index in all_buttons[buttons[i]].iter() {
                    b_indicators[index] = if b_indicators[index] == '.' { '#' } else { '.' };
                }
                let mut b_buttons = buttons.clone();
                b_buttons.remove(i);
                states.push_back((b_indicators, b_buttons));
            }
        }
    }
    println!("{}", total_presses);
}

pub fn solve_p2<F: BufRead>(input: F) {
    let mut total_presses = 0;
    for line in input.lines().map(|l| l.unwrap()) {
        let parts: Vec<&str> = line.split(' ').collect();
        let final_joltage: Vec<usize> = parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1]
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        let all_buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1]
            .iter()
            .map(|b| {
                b[1..b.len() - 1]
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .collect();

        print!(
            "{{{}}}: ",
            final_joltage
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        let mut states: VecDeque<(Vec<usize>, usize)> = VecDeque::new();
        states.push_back((final_joltage.iter().map(|_| 0).collect(), 0));
        let mut best_count: HashMap<Vec<usize>, usize> = HashMap::new();

        'state: while states.len() > 0 {
            let (joltage, depth) = states.pop_front().unwrap();
            if joltage == final_joltage {
                total_presses += depth;
                println!("{}", depth);
                break;
            }
            for i in 0..joltage.len() {
                if joltage[i] > final_joltage[i] {
                    continue 'state;
                }
            }
            if best_count.get(&joltage).is_some_and(|d| d <= &depth) {
                continue;
            }
            best_count.insert(joltage.clone(), depth);
            for button in all_buttons.iter() {
                let mut b_joltage = joltage.clone();
                for &index in button.iter() {
                    b_joltage[index] += 1;
                }
                states.push_back((b_joltage, depth + 1));
            }
        }
    }
    println!("{}", total_presses);
}
