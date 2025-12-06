use std::io::BufRead;

pub fn solve_p1<F: BufRead>(input: F) {
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut operands: Vec<char> = Vec::new();

    for l in input.lines() {
        let line = l.unwrap();
        let parts = line
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        if parts[0].parse::<u64>().is_ok() {
            numbers.push(parts.iter().map(|n| n.parse::<u64>().unwrap()).collect());
        } else {
            operands = parts.iter().map(|n| n.chars().nth(0).unwrap()).collect();
            break;
        }
    }

    let mut total = 0;
    for i in 0..operands.len() {
        let mut sub_total = if operands[i] == '*' { 1 } else { 0 };
        for n in 0..numbers.len() {
            match operands[i] {
                '*' => sub_total *= numbers[n][i],
                _ => sub_total += numbers[n][i],
            };
        }
        total += sub_total;
    }

    println!("{}", total);
}

pub fn solve_p2<F: BufRead>(input: F) {
    let sheet: Vec<Vec<char>> = input.lines().map(|l| l.unwrap().chars().collect()).collect();

    let mut total: u64 = 0;
    let mut sub_total: u64 = 0;

    let mut operand = ' ';
    for i in 0..sheet[0].len() {
        let mut number: u64 = 0;
        for d in 0..sheet.len() - 1 {
            if sheet[d][i] != ' ' {
                number = number * 10 + ((sheet[d][i] as u64) - ('0' as u64));
            }
        }
        if number == 0 {
            println!(" = {}", sub_total);
            total += sub_total;
            operand = ' ';
        } else {
            if operand == ' ' {
                operand = sheet[sheet.len() - 1][i];
                sub_total = if operand == '*' { 1 } else { 0 };
                print!("{}", operand);
            }
            print!(" {}", number);
            match operand {
                '*' => sub_total *= number,
                _ => sub_total += number,
            };
        }
    }
    println!(" = {}", sub_total);
    total += sub_total;

    println!("{}", total);
}
