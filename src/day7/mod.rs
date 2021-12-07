use std::cmp::min;

pub fn part_1(positions: &Vec<usize>) -> usize {
    // median of positions
    let middle = positions.len() / 2;
    let mut sorted_positions = positions.clone();
    sorted_positions.sort();
    let median = sorted_positions[middle];
    positions.iter().fold(0, |acc, x| {
        acc + ((median as isize) - (*x as isize)).abs() as usize
    })
}

pub fn part_2(positions: &Vec<usize>) -> usize {
    let mut sorted_positions = positions.clone();
    sorted_positions.sort();

    let mut min_fuel = fuel_at_position(&sorted_positions, 0);
    for i in 1..sorted_positions.len() {
        let current_fuel = fuel_at_position(&sorted_positions, i);
        if current_fuel <= min_fuel {
            min_fuel = current_fuel;
        } else {
           break
        }
    }
    min_fuel
}

pub fn fuel_at_position(sorted_positions: &Vec<usize>, position: usize) -> usize {
    sorted_positions.iter().fold(0, |acc, x| {
        let n = ((position as isize) - (*x as isize)).abs() as usize;
        acc +  n * (n + 1) / 2
    })
}

pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let parsed_input = super::parse_input(include_str!("./day7.sample.txt"));
        assert_eq!(37, super::part_1(&parsed_input));
    }

    #[test]
    fn test_part_2() {
        let parsed_input = super::parse_input(include_str!("./day7.sample.txt"));
        assert_eq!(168, super::part_2(&parsed_input));
    }

}
