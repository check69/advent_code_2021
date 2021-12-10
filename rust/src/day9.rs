use core::num;
use std::{collections::HashSet, str::FromStr};

struct Point {
    numbers: Vec<usize>,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Point {
            numbers: s
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>(),
        })
    }
}

fn check_number(numbers: Vec<Vec<usize>>, i: &usize, j: &usize) -> bool {
    let number: usize = numbers[*i][*j];
    if *i > 0_usize {
        if number >= numbers[i - 1][*j] {
            return false;
        }
    }
    if *i < numbers.len() - 1 {
        if number >= numbers[i + 1][*j] {
            return false;
        }
    }
    if *j > 0_usize {
        if number >= numbers[*i][j - 1] {
            return false;
        }
    }
    if *j < numbers[0].len() - 1 {
        if number >= numbers[*i][j + 1] {
            return false;
        }
    }
    true
}

fn recursive_adjacents(
    adjacents: &mut HashSet<(usize, usize)>,
    numbers: &Vec<Vec<usize>>,
    i: usize,
    j: usize,
) {
    let (mut curr_i, mut curr_j) = (i, j);
    if adjacents.contains(&(curr_i, curr_j)) || numbers[curr_i][curr_j] == 9 {
        return;
    }
    adjacents.insert((curr_i, curr_j));
    if curr_i > 0 {
        curr_i -= 1;
        //println!("i {}, j {} = {}", curr_i, curr_j, numbers[curr_i][curr_j]);
        if numbers[curr_i][j] != 9 {
            recursive_adjacents(adjacents, numbers, curr_i, j);
        }
    }
    curr_i = i;
    if curr_i < numbers.len() - 1 {
        curr_i += 1;
        if numbers[curr_i][j] != 9 {
            recursive_adjacents(adjacents, numbers, curr_i, j);
        }
    }
    if curr_j > 0 {
        curr_j -= 1;
        if numbers[i][curr_j] != 9 {
            recursive_adjacents(adjacents, numbers, i, curr_j);
        }
    }
    curr_j = j;
    if curr_j < numbers[0].len() - 1 {
        curr_j += 1;
        if numbers[i][curr_j] != 9 {
            recursive_adjacents(adjacents, numbers, i, curr_j);
        }
    }
}

fn get_adjacents(numbers: Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    let mut ret: usize = 0;
    let mut adjacents: HashSet<(usize, usize)> = HashSet::new();

    recursive_adjacents(&mut adjacents, &numbers, i, j);
    ret += adjacents.len();
    ret
}

#[cfg(test)]
mod test {
    use crate::{
        day9::{check_number, Point},
        utils,
    };
    use std::path::Path;

    use super::get_adjacents;

    #[test]
    fn example1() {
        let data: Vec<Point> = utils::read_data(Path::new("data/day9_example")).unwrap();
        let mut join_data: Vec<Vec<usize>> = vec![];
        for point in data {
            join_data.push(point.numbers);
        }

        let mut ret: Vec<usize> = vec![];
        for (i, row) in join_data.iter().enumerate() {
            for (j, number) in row.iter().enumerate() {
                if check_number(join_data.clone(), &i, &j) {
                    ret.push(number.clone());
                }
            }
        }

        println!("{}", ret.iter().sum::<usize>() + ret.len());
    }

    #[test]
    fn solution1() {
        let data: Vec<Point> = utils::read_data(Path::new("data/day9")).unwrap();
        let mut join_data: Vec<Vec<usize>> = vec![];
        for point in data {
            join_data.push(point.numbers);
        }

        let mut ret: Vec<usize> = vec![];
        for (i, row) in join_data.iter().enumerate() {
            for (j, number) in row.iter().enumerate() {
                if check_number(join_data.clone(), &i, &j) {
                    ret.push(number.clone());
                }
            }
        }

        println!("{} - {}", ret.iter().sum::<usize>() + ret.len(), ret.len());
    }

    #[test]
    fn example2() {
        let data: Vec<Point> = utils::read_data(Path::new("data/day9_example")).unwrap();
        let mut join_data: Vec<Vec<usize>> = vec![];
        for point in data {
            join_data.push(point.numbers);
        }

        let mut ret: Vec<usize> = vec![];
        let mut adjacents: Vec<usize> = vec![];
        for (i, row) in join_data.iter().enumerate() {
            for (j, number) in row.iter().enumerate() {
                if check_number(join_data.clone(), &i, &j) {
                    ret.push(get_adjacents(join_data.clone(), i, j));
                }
            }
        }

        ret.sort();
        ret.reverse();

        println!("{:?}", ret);
        println!(
            "{} * {} * {} = {}",
            ret[0],
            ret[1],
            ret[2],
            ret[0] * ret[1] * ret[2]
        );
    }

    #[test]
    fn solution2() {
        let data: Vec<Point> = utils::read_data(Path::new("data/day9")).unwrap();
        let mut join_data: Vec<Vec<usize>> = vec![];
        for point in data {
            join_data.push(point.numbers);
        }

        let mut ret: Vec<usize> = vec![];
        let mut adjacents: Vec<usize> = vec![];
        for (i, row) in join_data.iter().enumerate() {
            for (j, number) in row.iter().enumerate() {
                if check_number(join_data.clone(), &i, &j) {
                    ret.push(get_adjacents(join_data.clone(), i, j));
                }
            }
        }

        ret.sort();
        ret.reverse();

        println!("{:?}", ret);
        println!(
            "{} * {} * {} = {}",
            ret[0],
            ret[1],
            ret[2],
            ret[0] * ret[1] * ret[2]
        );
    }
}
