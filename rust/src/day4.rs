use std::{
    io::{BufRead, BufReader, Read},
    str::FromStr,
    vec,
};

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<Vec<usize>>,
    marked_numbers: Vec<Vec<bool>>,
}

impl Board {
    fn from_string_vec(lines: Vec<String>) -> Self {
        let mut board = Board {
            numbers: vec![],
            marked_numbers: vec![],
        };
        for line in lines {
            let mut bingo_line: Vec<usize> = vec![];
            bingo_line.append(
                &mut line
                    .split(" ")
                    .map(|c| c.trim())
                    .filter(|c| !c.is_empty())
                    .map(|c| c.parse().unwrap())
                    .collect::<Vec<_>>(),
            );
            let length = bingo_line.len();
            board.numbers.push(bingo_line);
            board.marked_numbers.push(vec![false; length]);
        }
        board
    }

    fn find_number(&mut self, number: usize) -> bool {
        for (i, line) in self.numbers.iter_mut().enumerate() {
            for (j, n) in line.iter().enumerate() {
                if *n == number {
                    self.numbers[i][j] = 0;
                    self.marked_numbers[i][j] = true;
                    return true;
                }
            }
        }

        false
    }

    fn check_rows(&self) -> bool {
        self.marked_numbers
            .iter()
            .any(|line| line.iter().all(|value| *value == true))
    }

    fn check_columns(&self) -> bool {
        (0..self.marked_numbers.len()).any(|column| {
            self.marked_numbers
                .iter()
                .map(|line| line[column])
                .all(|value| value == true)
        })
    }

    fn check_board(&self) -> bool {
        self.check_rows() || self.check_columns()
    }

    fn sum_numbers(&mut self) -> usize {
        let mut total: usize = 0;
        for line in self.numbers.iter() {
            total += line.iter().sum::<usize>();
        }

        total
    }

    fn print_board(&self) {
        for line in self.numbers.clone() {
            println!("{:?}", line);
        }
        for line in self.marked_numbers.clone() {
            println!("{:?}", line);
        }
    }
}

struct Boards {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

impl Boards {
    fn from_reader<T>(buffer: &mut BufReader<T>) -> Result<Self, ()>
    where
        T: Read,
    {
        let mut ret: Boards = Boards {
            numbers: vec![],
            boards: vec![],
        };
        let mut line = String::new();
        let bytes_read = buffer.read_line(&mut line);
        match bytes_read {
            Ok(bytes) => {
                if bytes != 0 {
                    ret.numbers = line
                        .split(",")
                        .map(|c| c.trim())
                        .filter(|c| !c.is_empty())
                        .map(|c| c.parse().unwrap())
                        .collect()
                }
            }
            Err(error) => {}
        }

        loop {
            let mut lines: Vec<String> = vec![];
            let mut line = String::new();
            buffer.read_line(&mut line).unwrap();
            for _ in 0..5 {
                let mut line = String::new();
                let bytes_read = buffer.read_line(&mut line);
                match bytes_read {
                    Ok(bytes) => {
                        if bytes != 0 {
                            lines.push(line);
                        } else {
                            return Ok(ret);
                        }
                    }
                    Err(error) => {
                        return Err(());
                    }
                }
            }
            ret.boards.push(Board::from_string_vec(lines));
        }
    }

    fn find_number(&mut self, number: usize) -> Option<Board> {
        for board in self.boards.iter_mut() {
            if board.find_number(number) {
                if board.check_board() {
                    return Some(board.clone());
                }
            }
        }

        None
    }
    
    fn find_last_number(&mut self, number: usize) -> Option<Board> {
        let mut ret: Option<Board> = None;
        let mut remove_index: Vec<usize> = vec![];
        for (i, board) in self.boards.iter_mut().enumerate() {
            if board.find_number(number) {
                if board.check_board() {
                    ret = Some(board.clone());
                    remove_index.push(i);
                }
            }
        }

        remove_index.reverse();
        for i in remove_index {
            self.boards.remove(i);
        }

        ret
    }

    fn print_boards(&self) {
        for board in self.boards.clone() {
            board.print_board();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        day4::Boards,
        utils::{self, get_lines},
    };
    use std::{path::Path, vec};

    use super::Board;

    #[test]
    fn example1() {
        let mut boards: Boards =
            Boards::from_reader(&mut get_lines(Path::new("data/day4")).unwrap()).unwrap();

        for number in boards.numbers.clone() {
            let mut board = boards.find_number(number);
            println!("++++++++++++++++++++++++++++++++++");
            match board {
                Some(mut board) => {
                    println!("{}", board.sum_numbers());
                    println!("{}", number);
                    println!("{}", board.sum_numbers() * number);
                    break;
                }
                None => {
                    continue;
                }
            }
        }
    }

    #[test]
    fn example2() {
        let mut boards: Boards =
            Boards::from_reader(&mut get_lines(Path::new("data/day4")).unwrap()).unwrap();
        
        let mut board: Board = Board{numbers: vec![], marked_numbers: vec![]};
        let mut the_number: usize = 0;
        for number in boards.numbers.clone() {
            match boards.find_last_number(number) {
                Some(last_board) => {
                    board = last_board;
                    the_number = number;
                }
                None => {}
            }
        }

        board.print_board();
        println!("{}", board.sum_numbers());
        println!("{}", the_number);
        println!("{}", board.sum_numbers() * the_number);
    }

    #[test]
    fn example1_improve() {}

    #[test]
    fn example2_improve() {}
}
