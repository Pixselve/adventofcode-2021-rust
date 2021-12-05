use crate::day4::parse_input;

mod read_file;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let string_data = include_str!("./day5/day5.input.txt");
    let result = day5::parse_input(string_data);
    println!("{:?}", day5::part_1(&result));
    println!("{:?}", day5::part_2(&result));


}

