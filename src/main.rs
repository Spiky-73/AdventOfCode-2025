use std::io::BufReader;

mod day01_secret_entrance;
mod day02_gift_shop;
mod day03_lobby;
mod day04_printing_department;
mod day05_cafeteria;
mod day06_trash_compactor;
mod day07_laboratories;
mod day08_playground;

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("no day given")
        .parse::<u8>()
        .expect("expected an number");
    let part = std::env::args()
        .nth(2)
        .expect("no part given")
        .parse::<u8>()
        .expect("expected an number");

    match day * 10 + part {
        011 => day01_secret_entrance::solve_p1(BufReader::new(std::io::stdin())),
        012 => day01_secret_entrance::solve_p2(BufReader::new(std::io::stdin())),
        021 => day02_gift_shop::solve_p1(BufReader::new(std::io::stdin())),
        022 => day02_gift_shop::solve_p2(BufReader::new(std::io::stdin())),
        031 => day03_lobby::solve_p1(BufReader::new(std::io::stdin())),
        032 => day03_lobby::solve_p2(BufReader::new(std::io::stdin())),
        041 => day04_printing_department::solve_p1(BufReader::new(std::io::stdin())),
        042 => day04_printing_department::solve_p2(BufReader::new(std::io::stdin())),
        051 => day05_cafeteria::solve_p1(BufReader::new(std::io::stdin())),
        052 => day05_cafeteria::solve_p2(BufReader::new(std::io::stdin())),
        061 => day06_trash_compactor::solve_p1(BufReader::new(std::io::stdin())),
        062 => day06_trash_compactor::solve_p2(BufReader::new(std::io::stdin())),
        071 => day07_laboratories::solve_p1(BufReader::new(std::io::stdin())),
        072 => day07_laboratories::solve_p2(BufReader::new(std::io::stdin())),
        081 => day08_playground::solve_p1(BufReader::new(std::io::stdin())),
        082 => day08_playground::solve_p2(BufReader::new(std::io::stdin())),
        _ => panic!("Not solution for this day yet"),
    };
}
