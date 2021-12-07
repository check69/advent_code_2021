use std::str::FromStr;

#[derive(Clone, Debug)]
struct Crab {
    positions: Vec<usize>,
}

impl FromStr for Crab {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions: Vec<usize> = s.split(",").map(|n| n.trim().parse().unwrap()).collect();
        Ok(Crab {
            positions: positions,
        })
    }
}

fn get_fuel(number: usize, target_number: usize) -> usize {
    (number as i32 - target_number as i32).abs() as usize
}

fn get_fuel_sum(number: usize, target_number: usize) -> usize {
    let x = (number as i32 - target_number as i32).abs() as usize;
    x * (x + 1) / 2 as usize
}

#[cfg(test)]
mod test {
    use crate::day7::{get_fuel, get_fuel_sum, Crab};
    use crate::utils;
    use std::array;
    use std::{path::Path, usize::MAX};

    #[test]
    fn example1() {
        let mut data: Vec<Crab> = utils::read_data(Path::new("data/day7_example")).unwrap();
        let mut positions = data[0].clone().positions;

        positions.sort();

        let mut dynamic: Vec<usize> = vec![0; positions.last().copied().unwrap()];

        for current_pos in 0..dynamic.len() {
            for pos in positions.iter() {
                dynamic[current_pos] += get_fuel(*pos, current_pos);
            }
        }

        println!("{}", dynamic.iter().min().unwrap());
    }

    #[test]
    fn solution1() {
        let mut data: Vec<Crab> = utils::read_data(Path::new("data/day7")).unwrap();
        let mut positions = data[0].clone().positions;

        positions.sort();

        let mut dynamic: Vec<usize> = vec![0; positions.last().copied().unwrap()];

        for current_pos in 0..dynamic.len() {
            for pos in positions.iter() {
                dynamic[current_pos] += get_fuel(*pos, current_pos);
            }
        }

        println!("{}", dynamic.iter().min().unwrap());
    }

    #[test]
    fn example2() {
        let mut data: Vec<Crab> = utils::read_data(Path::new("data/day7_example")).unwrap();
        let mut positions = data[0].clone().positions;

        positions.sort();

        let mut dynamic: Vec<usize> = vec![0; positions.last().copied().unwrap()];

        for current_pos in 0..dynamic.len() {
            for pos in positions.iter() {
                dynamic[current_pos] += get_fuel_sum(*pos, current_pos);
            }
        }

        println!("{}", dynamic.iter().min().unwrap());
    }

    #[test]
    fn solution2() {
        let mut data: Vec<Crab> = utils::read_data(Path::new("data/day7")).unwrap();
        let mut positions = data[0].clone().positions;

        positions.sort();

        let mut dynamic: Vec<usize> = vec![0; positions.last().copied().unwrap()];

        for current_pos in 0..dynamic.len() {
            for pos in positions.iter() {
                dynamic[current_pos] += get_fuel_sum(*pos, current_pos);
            }
        }

        println!("{}", dynamic.iter().min().unwrap());
    }
}
