use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Add;

pub fn parse_input(input: &str) -> Vec<(Coordinate, Coordinate)> {
    input
        .lines()
        .map(|line| {
            let splitted = line.split(" -> ").collect::<Vec<&str>>();
            (Coordinate::new(splitted[0]), Coordinate::new(splitted[1]))
        })
        .collect::<Vec<(Coordinate, Coordinate)>>()
}

pub fn part_1(data: &Vec<(Coordinate, Coordinate)>) -> usize {
    let vertical_horizontal = data
        .iter()
        .filter(|(from, to)| from.x == to.x || from.y == to.y)
        .collect::<Vec<&(Coordinate, Coordinate)>>();
    let mut vertical_horizontal_map: HashMap<Coordinate, u32> = HashMap::new();

    for (from, to) in vertical_horizontal {
        if from.y == to.y {
            for x in min(from.x, to.x)..max(to.x + 1, from.x + 1) {
                match vertical_horizontal_map.get_mut(&Coordinate { x, y: to.y }) {
                    None => {
                        vertical_horizontal_map.insert(Coordinate { x, y: to.y }, 1);
                    }
                    Some(value) => {
                        *value += 1;
                    }
                }
            }
        } else if from.x == to.x {
            for y in min(from.y, to.y)..max(to.y + 1, from.y + 1) {
                match vertical_horizontal_map.get_mut(&Coordinate { x: from.x, y }) {
                    None => {
                        vertical_horizontal_map.insert(Coordinate { x: from.x, y }, 1);
                    }
                    Some(value) => {
                        *value += 1;
                    }
                }
            }
        }
    }

    vertical_horizontal_map
        .values()
        .filter(|x| x >= &&2_u32)
        .count()
}

pub fn part_2(data: &Vec<(Coordinate, Coordinate)>) -> usize {
    let mut vertical_horizontal_map: HashMap<Coordinate, u32> = HashMap::new();
    for (from, to) in data {
        if from.y == to.y {
            for x in min(from.x, to.x)..max(to.x + 1, from.x + 1) {
                match vertical_horizontal_map.get_mut(&Coordinate { x, y: to.y }) {
                    None => {
                        vertical_horizontal_map.insert(Coordinate { x, y: to.y }, 1);
                    }
                    Some(value) => {
                        *value += 1;
                    }
                }
            }
        } else if from.x == to.x {
            for y in min(from.y, to.y)..max(to.y + 1, from.y + 1) {
                match vertical_horizontal_map.get_mut(&Coordinate { x: from.x, y }) {
                    None => {
                        vertical_horizontal_map.insert(Coordinate { x: from.x, y }, 1);
                    }
                    Some(value) => {
                        *value += 1;
                    }
                }
            }
        } else {
            let x_sign = (to.x - from.x) / (to.x - from.x).abs();
            let y_sign = (to.y - from.y) / (to.y - from.y).abs();

            for i in 0..=(from.x - to.x).abs() {
                match vertical_horizontal_map.get_mut(&Coordinate {
                    x: from.x + i * x_sign,
                    y: from.y + i * y_sign,
                }) {
                    None => {
                        vertical_horizontal_map.insert(
                            Coordinate {
                                x: from.x + i * x_sign,
                                y: from.y + i * y_sign,
                            },
                            1,
                        );
                    }
                    Some(value) => {
                        *value += 1;
                    }
                }
            }
        }
    }
    vertical_horizontal_map
        .values()
        .filter(|x| x >= &&2_u32)
        .count()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(data: &str) -> Self {
        let mut split = data
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Self {
            x: split[0],
            y: split[1],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::Coordinate;

    #[test]
    fn test_parse_input() {
        let test_string = "0,9 -> 5,9\n8,0 -> 0,8";
        let expected = vec![
            (Coordinate::new("0,9"), Coordinate::new("5,9")),
            (Coordinate::new("8,0"), Coordinate::new("0,8")),
        ];

        assert_eq!(expected, super::parse_input(test_string));
    }

    #[test]
    fn test_part_1() {
        let string_data = include_str!("./day5.sample.txt");
        let result = super::parse_input(string_data);

        assert_eq!(5, super::part_1(&result));
    }

    #[test]
    fn test_part_2() {
        let string_data = include_str!("./day5.sample.txt");
        let result = super::parse_input(string_data);

        assert_eq!(12, super::part_2(&result));
    }
}
