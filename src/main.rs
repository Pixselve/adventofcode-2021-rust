use crate::day4::parse_input;

mod read_file;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let (numbers, boards) = parse_input(include_str!("./day4/day4.input.txt"));
    println!("{:?}", day4::part_1(numbers.clone(), boards.clone()));
    println!("{:?}", day4::part_2(numbers, boards));


}

