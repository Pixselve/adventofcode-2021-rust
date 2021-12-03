use std::borrow::Borrow;
use std::collections::HashMap;

pub fn part_1(input: &String) -> i32 {
    let strings = input.lines().collect::<Vec<&str>>();

    let mut one_count: HashMap<usize, usize> = HashMap::new();
    for x in &strings {
        for i in 0..x.len() {
            if x.chars().nth(i).unwrap() == '1' {
                let reverse_i = x.len() - i - 1;

                if one_count.contains_key(&reverse_i) {
                    *one_count.get_mut(&reverse_i).unwrap() += 1;
                } else {
                    one_count.insert(reverse_i, 1);
                }
            }
        }
    }
    let mut most_numbers = 0;
    let mut least_number = 0;

    for (k, v) in &one_count {
        if v > &(&strings.len() / 2) {
            most_numbers += 2_i32.pow(k.clone() as u32) * 1;
        } else {
            least_number += 2_i32.pow(k.clone() as u32) * 1;
        }
    }
    return most_numbers * least_number;
}

pub fn part_2(input: &String) -> i32 {
    let strings = input
        .lines()
        .map(|x| BinaryNumber::new(x))
        .collect::<Vec<BinaryNumber>>();
    ();
    let mut most_numbers: Vec<BinaryNumber> = strings.clone();
    let mut bit_position = 0;

    while most_numbers.len() > 1 {
        let mut ones: Vec<BinaryNumber> = vec![];
        let mut zeros: Vec<BinaryNumber> = vec![];
        for x in most_numbers {
            if (x.clone()).get_at_index(bit_position) == 1 {
                ones.push(x.clone());
            } else {
                zeros.push(x.clone());
            }
        }
        if ones.len() >= zeros.len() {
            most_numbers = ones;
        } else {
            most_numbers = zeros;
        }
        bit_position += 1;
    }
    let mut least_numbers: Vec<BinaryNumber> = strings.clone();
    bit_position = 0;

    while least_numbers.len() > 1 {
        let mut ones: Vec<BinaryNumber> = vec![];
        let mut zeros: Vec<BinaryNumber> = vec![];
        for x in least_numbers {
            if (x.clone()).get_at_index(bit_position) == 1 {
                ones.push(x.clone());
            } else {
                zeros.push(x.clone());
            }
        }
        if ones.len() < zeros.len() {
            least_numbers = ones;
        } else {
            least_numbers = zeros;
        }
        bit_position += 1;
    }

    return most_numbers[0].convert_to_decimal() * least_numbers[0].convert_to_decimal();
}

#[derive( Clone)]
struct BinaryNumber {
    number: Vec<i32>,
}

impl BinaryNumber {
    fn new(line: &str) -> BinaryNumber {

        BinaryNumber {
            number: line
                .chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>(),
        }
    }
    fn get_at_index(&self, index: usize) -> i32 {
        self.number[index]
    }
    fn convert_to_decimal(&self) -> i32 {
        let mut decimal: i32 = 0;
        for i in 0..self.number.len() {
            decimal += self.number[self.number.len() - i - 1] * 2_i32.pow(i as u32);
        }
        return decimal;
    }
}
