mod read_file;
mod day1;
mod day2;

fn main() {
    let input = read_file::read_file("src/day2/day2.input.txt");
    if input.is_ok() {
        println!("{:?}", day2::part_1(day2::parse_input(input.as_ref().unwrap()) ));
        println!("{:?}", day2::part_2(day2::parse_input(input.as_ref().unwrap()) ));
    } else {
        println!("{}", input.unwrap_err());
    }

}

