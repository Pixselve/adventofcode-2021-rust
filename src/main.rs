use std::time::Instant;
use crate::day4::parse_input;

mod read_file;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let string_data = include_str!("./day7/day7.input.txt");
    let result = day7::parse_input(string_data);
    println!("{:?}", day7::part_1(&result));
    let now = Instant::now();
    println!("{:?}", day7::part_2(&result));
    println!("{}", now.elapsed().as_millis());

}

