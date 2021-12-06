use std::{
    collections::HashMap,
    fmt::{self, Display},
    iter::Map,
    ops::Range,
    str::FromStr,
    thread::spawn,
    vec,
};

#[derive(Clone, Debug)]
struct LanternFish {
    fishes: Vec<usize>,
}

impl FromStr for LanternFish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fishes: Vec<usize> = s.split(",").map(|n| n.trim().parse().unwrap()).collect();
        Ok(LanternFish { fishes: fishes })
    }
}

fn spawn_lanternfish(life: &mut Vec<usize>) {
    let mut newborns: Vec<usize> = vec![];
    for lanternfish in life.iter_mut() {
        if *lanternfish == 0 {
            *lanternfish = 6;
            newborns.push(8);
        } else {
            *lanternfish -= 1_usize;
        }
    }

    life.append(&mut newborns);
}

fn original_state(fishes: &Vec<usize>) -> Vec<usize> {
    let mut state: Vec<usize> = vec![0; 9];
    for fish in fishes.into_iter() {
        state[*fish] += 1;
    }

    state
}

fn spawn_fishes(state: &mut Vec<usize>, target_day: usize) {
    for day in 0..target_day {
        let x = state.remove(0);
        state.push(x);
        state[6] += state[8];
    }
}

#[cfg(test)]
mod test {
    use crate::{
        day6::{original_state, spawn_fishes, LanternFish},
        utils,
    };
    use std::path::Path;

    use super::spawn_lanternfish;

    #[test]
    fn example1() {
        let mut data: Vec<LanternFish> = utils::read_data(Path::new("data/day6_example")).unwrap();
        for _ in 0..80 {
            spawn_lanternfish(&mut data[0].fishes);
        }

        println!("{}", data[0].fishes.len());
    }

    #[test]
    fn solution1() {
        let mut data: Vec<LanternFish> = utils::read_data(Path::new("data/day6")).unwrap();
        for _ in 0..80 {
            spawn_lanternfish(&mut data[0].fishes);
        }

        println!("{}", data[0].fishes.len());
    }

    #[test]
    fn example2() {
        let mut data: Vec<LanternFish> = utils::read_data(Path::new("data/day6_example")).unwrap();
        let mut fishes: Vec<usize> = original_state(&data[0].fishes);

        spawn_fishes(&mut fishes, 256);

        println!("{}", fishes.iter().sum::<usize>());
    }

    #[test]
    fn solution2() {
        let mut data: Vec<LanternFish> = utils::read_data(Path::new("data/day6")).unwrap();
        let mut fishes: Vec<usize> = original_state(&data[0].fishes);

        spawn_fishes(&mut fishes, 256);

        println!("{}", fishes.iter().sum::<usize>());
    }
}
