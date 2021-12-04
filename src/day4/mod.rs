use std::collections::HashSet;

pub fn part_1(numbers: Vec<u8>, boards: Vec<Board>) -> u32 {
    let mut marked_numbers: HashSet<u8> = HashSet::new();
    for x in numbers {
        marked_numbers.insert(x);
        let winning_board = boards.iter().find(|b| b.is_win(&|x| marked_numbers.contains(x)));
        if let Some(board) = winning_board {
            return (board.sum_unmarked(&|x| marked_numbers.contains(x)) as u32) * (x as u32);
        }
    }
    0
}

pub fn part_2(numbers: Vec<u8>, boards: Vec<Board>) -> u32 {
    let mut marked_numbers: HashSet<u8> = HashSet::new();
    let mut new_boards: Vec<Board> = boards;

    for number in numbers {

        marked_numbers.insert(number);

        if new_boards.len() == 1 {
            let board = new_boards.first().unwrap();
            if board.is_win(&|x| marked_numbers.contains(x)) {
                return (board.sum_unmarked(&|x| marked_numbers.contains(x)) as u32) * (number as u32);
            }

        } else {
            new_boards = new_boards.iter().filter(|b| !b.is_win(&|x| marked_numbers.contains(x))).cloned().collect::<Vec<Board>>();
        }
    }


    0

}


pub fn parse_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let lines = input.split("\n\n").collect::<Vec<&str>>();
    let numbers = lines[0].split(',').map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    let boards = lines[1..].iter().map(|board_string| Board::new(board_string)).collect::<Vec<Board>>();
    (numbers, boards)
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    grid: Vec<Vec<u8>>,
}

impl Board {
    ///
    /// Create a new board from the string :
    ///
    /// 22 13 17 11  0
    ///  8  2 23  4 24
    /// 21  9 14 16  7
    ///  6 10  3 18  5
    ///  1 12 20 15 19
    pub fn new(input: &str) -> Board {
        let lines = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();
        Board { grid: lines }
    }
    pub fn is_win(&self, is_marked: &dyn Fn(&u8) -> bool) -> bool {
        // check rows
        for grid_line in &self.grid {
            let mut is_win = true;
            for cell in grid_line {
                if !is_marked(cell) {
                    is_win = false;
                    break;
                }
            }
            if is_win {
                return true;
            }
        }

        // check columns
        for i in 0..self.grid.len() {
            let mut is_win = true;
            for j in 0..self.grid.len() {
                if !is_marked(&self.grid[j][i]) {
                    is_win = false;
                    break;
                }
            }
            if is_win {
                return true;
            }
        }

        false
    }

    pub fn sum_unmarked(&self, is_marked: &dyn Fn(&u8) -> bool) -> u32 {
        let mut sum: u32 = 0;
        for grid_line in &self.grid {
            for cell in grid_line {
                if !is_marked(cell) {
                    sum += *cell as u32;
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::{Board, parse_input, part_1, part_2};
    use std::collections::HashSet;

    #[test]
    fn it_works() {
        let board = Board::new(
            "22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19",
        );
        assert_eq!(
            board.grid,
            vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ]
        );
    }

    #[test]
    fn line_win() {
        let board = Board::new(
            "22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19",
        );
        let mut marked_values: HashSet<u8> = HashSet::new();
        // mark first line as marked
        marked_values.insert(22);
        marked_values.insert(13);
        marked_values.insert(17);
        marked_values.insert(11);
        marked_values.insert(0);

        assert!(board.is_win(&|cell| marked_values.contains(cell)));
    }

    #[test]
    fn column_win() {
        let board = Board::new(
            "22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19",
        );
        let mut marked_values: HashSet<u8> = HashSet::new();
        // mark first line as marked
        marked_values.insert(22);
        marked_values.insert(8);
        marked_values.insert(21);
        marked_values.insert(6);
        marked_values.insert(1);

        assert!(board.is_win(&|cell| marked_values.contains(cell)));
    }

    #[test]
    fn test_parse() {
        let result = parse_input(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
        );


        assert_eq!(result.0, vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1]);
        assert_eq!(result.1, vec![
            Board::new("22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19"),
            Board::new(" 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6"),
            Board::new("14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7"),
        ]);
    }

    #[test]
    fn test_part_1() {
        let result = parse_input(include_str!("./day4.sample.txt"));

        assert_eq!(part_1(result.0, result.1), 4512);
    }

    #[test]
    fn test_part_2() {
        let result = parse_input(include_str!("./day4.sample.txt"));

        assert_eq!(part_2(result.0, result.1), 1924);
    }
}
