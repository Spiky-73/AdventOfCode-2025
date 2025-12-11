use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
    io::BufRead,
};

#[derive(Eq, PartialEq)]
struct State {
    estimated_weight: usize,
    weight: usize,
    state: Vec<usize>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimated_weight.cmp(&self.estimated_weight)
    }
}

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
        let end_state: Vec<usize> = parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1]
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
            end_state
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        let mut states: BinaryHeap<State> = BinaryHeap::new();
        states.push(State {
            estimated_weight: 0,
            weight: 0,
            state: end_state.iter().map(|_| 0).collect(),
        });
        let mut weights: HashMap<Vec<usize>, usize> = HashMap::new();

        while let Some(State { weight, state, .. }) = states.pop() {
            if state == end_state {
                total_presses += weight;
                println!("{}", weight);
                break;
            }
            'state: for button in all_buttons.iter() {
                let mut next_state = state.clone();
                for &index in button.iter() {
                    next_state[index] += 1;
                }
                let weight = weight + 1;
                for i in 0..next_state.len() {
                    if next_state[i] > end_state[i] {
                        continue 'state;
                    }
                }
                if weights.get(&next_state).is_some_and(|&w| weight < w) {
                    continue;
                }
                let estimated_weight: usize =
                    weight + end_state.iter().sum::<usize>() - next_state.iter().sum::<usize>();

                weights.insert(next_state.clone(), weight);
                states.push(State {
                    estimated_weight: estimated_weight,
                    weight: weight,
                    state: next_state,
                });
            }
        }
    }
    println!("{}", total_presses);
}
