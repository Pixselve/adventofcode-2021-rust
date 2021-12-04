#[derive(Debug)]
pub enum Direction {
    Foward,
    Down,
    Up,
}

#[derive(Debug)]
pub struct Move {
    direction: Direction,
    steps: i32,
}

pub fn part_1(moves: Vec<Move>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for m in moves {
        match m.direction {
            Direction::Foward => {
                horizontal += m.steps;
            }
            Direction::Down => {
                depth += m.steps;
            }
            Direction::Up => {
                depth -= m.steps;
            }
        }
    }
    return horizontal * depth;
}

pub fn part_2(moves: Vec<Move>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for m in moves {
        match m.direction {
            Direction::Foward => {
                horizontal += m.steps;
                depth += aim * m.steps;
            }
            Direction::Down => {
                aim += m.steps;
            }
            Direction::Up => {
                aim -= m.steps;
            }
        }
    }
    return horizontal * depth;
}

pub fn parse_input(input: &String) -> Vec<Move> {
    input
        .split("\r\n")
        .map(|x| {
            let splitted = x.split(' ').collect::<Vec<&str>>();
            let direction = match splitted[0] {
                "forward" => Direction::Foward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("Unknown direction"),
            };
            Move {
                direction,
                steps: splitted[1].parse::<i32>().unwrap(),
            }
        })
        .collect()
}
