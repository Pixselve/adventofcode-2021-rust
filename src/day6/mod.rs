pub fn part_1(squids: &[usize; 9]) -> usize {
    squids_after_day(squids, 80)
}

pub fn part_2(squids: &[usize; 9]) -> usize {
    squids_after_day(squids, 256)
}

fn squids_after_day(squids: &[usize; 9], day: usize) -> usize {
    let mut squids_array = squids.clone();
    for _ in 0..day {
        let zero_count = squids_array[0];
        for j in 0..(squids_array.len() - 1) {
            squids_array[j] = squids_array[j + 1];
            squids_array[j + 1] = 0;
        }
        squids_array[6] += zero_count;
        squids_array[8] += zero_count;
    }
    println!("{:?}", squids_array);
    squids_array.iter().sum()
}

pub fn parse_input(input: &str) -> Vec<u8> {
    input.split(',').map(|s| s.parse::<u8>().unwrap()).collect()
}

pub fn parse_input_array(input: &str) -> [usize; 9] {
    let mut array: [usize; 9] = [0; 9];
    input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|value| array[value] += 1);
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&[0, 1, 1, 2, 1, 0, 0, 0, 0]), 5934);
    }
}
