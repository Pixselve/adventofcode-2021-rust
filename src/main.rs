mod read_file;
mod day1;
mod day2;
mod day3;

fn main() {
    let input = read_file::read_file("src/day3/day3.input.txt");
    if input.is_ok() {
        println!("{:?}", day3::part_1(input.as_ref().unwrap() ));
        println!("{:?}", day3::part_2(input.as_ref().unwrap() ));
    } else {
        println!("{}", input.unwrap_err());
    }

}

