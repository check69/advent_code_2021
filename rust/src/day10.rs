use std::collections::HashMap;

static CHARACTERS_OPEN: [char; 4] = ['(', '[', '<', '{'];
static CHARACTERS_CLOSE: [char; 4] = [')', ']', '>', '}'];

fn found_illegal(line: &String) -> Option<char> {
    let mut chunck_open: Vec<char> = vec![];
    for character in line.chars() {
        if CHARACTERS_OPEN.contains(&character) {
            chunck_open.push(character);
        } else {
            match chunck_open.pop() {
                Some(element) => {
                    let index = CHARACTERS_CLOSE
                        .iter()
                        .position(|&r| r == character)
                        .unwrap();
                    if element != CHARACTERS_OPEN[index] {
                        return Some(character);
                    }
                }
                None => {
                    return Some(character);
                }
            }
        }
    }

    None
}

fn fill_uncomplete(line: &String) -> usize {
    let mut total: usize = 0;
    let mut chunck_open: Vec<char> = vec![];
    for character in line.chars() {
        if CHARACTERS_OPEN.contains(&character) {
            chunck_open.push(character);
        } else {
            match chunck_open.pop() {
                Some(element) => {
                    let index = CHARACTERS_CLOSE
                        .iter()
                        .position(|&r| r == character)
                        .unwrap();
                    if element != CHARACTERS_OPEN[index] {
                        return total;
                    }
                }
                None => {
                }
            }
        }
    }

    let mut y: usize = chunck_open.iter().rev().fold(0, | r |)
    let mut x = 0;
    for uncomplete in chunck_open.iter().rev() {
        x = if *uncomplete == '(' {
            1
        } else if *uncomplete == '[' {
            2
        } else if *uncomplete == '{' {
            3
        } else if *uncomplete == '<' {
            4
        }
        else { unreachable!("paniquea") }
    };

    total * 5 + x
}

fn sum_illegal_parts(illegals: Vec<char>) -> usize {
    let mut total: usize = 0;
    for character in illegals.iter() {
        if *character == ')' {
            total += 3;
        } else if *character == ']' {
            total += 57;
        } else if *character == '}' {
            total += 1197;
        } else if *character == '>' {
            total += 25137;
        }
    }

    total
}

#[cfg(test)]
mod test {
    use crate::{
        day10::{fill_uncomplete, sum_illegal_parts},
        utils,
    };
    use std::path::Path;

    use super::found_illegal;

    #[test]
    fn example1() {
        let data: Vec<String> = utils::read_data(Path::new("data/day10_example")).unwrap();
        let mut ret: Vec<char> = vec![];
        for line in data.iter() {
            if let Some(character) = found_illegal(&line) {
                ret.push(character);
            }
        }
        assert_eq!(sum_illegal_parts(ret), 26397);
    }

    #[test]
    fn solution1() {
        let data: Vec<String> = utils::read_data(Path::new("data/day10")).unwrap();
        let mut ret: Vec<char> = vec![];
        for line in data {
            if let Some(character) = found_illegal(&line) {
                ret.push(character);
            }
        }

        println!("{:?}", sum_illegal_parts(ret));
    }

    #[test]
    fn example2() {
        let mut data: Vec<String> = utils::read_data(Path::new("data/day10_example")).unwrap();
        let mut ret: Vec<usize> = vec![];
        for (index, line) in data.iter().enumerate() {
            if let Some(character) = found_illegal(&line) {
                ret.push(index);
            }
        }

        for index in ret.iter().rev() {
            data.remove(*index);
        }

        let mut ret: Vec<usize> = vec![];

        for line in data {
            ret.push(fill_uncomplete(&line));
        }

        ret.sort();
        println!("{:?}", ret );
        assert_eq!(ret[(ret.len() - 1) / 2], 288957);
    }

    #[test]
    fn solution2() {
        let mut data: Vec<String> = utils::read_data(Path::new("data/day10")).unwrap();
        let mut ret: Vec<usize> = vec![];
        for (index, line) in data.iter().enumerate() {
            if let Some(character) = found_illegal(&line) {
                ret.push(index);
            }
        }

        for index in ret.iter().rev() {
            data.remove(*index);
        }

        let mut ret: Vec<usize> = vec![];

        for line in data {
            ret.push(fill_uncomplete(&line));
        }

        ret.sort();
        println!("{:?}", ret[(ret.len() -1) / 2] );
    }
}
