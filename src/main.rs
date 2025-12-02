use std::io::BufReader;

mod day01_secret_entrance;
mod day02_gift_shop;

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
        _ => panic!("Not solution for this day yet"),
    };
}
