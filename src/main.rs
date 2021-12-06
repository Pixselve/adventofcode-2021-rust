use crate::day4::parse_input;

mod read_file;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let string_data = include_str!("./day6/day6.input.txt");
    let result = day6::parse_input_array(string_data);
    println!("{:?}", day6::part_1(&result));
    println!("{:?}", day6::part_2(&result));


}

