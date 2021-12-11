use std::str::FromStr;

static SURROUND: [(isize, isize); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

struct HeatMap(Vec<Vec<Octopus>>);

impl FromStr for HeatMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|l| {
                    l.trim()
                        .chars()
                        .map(|c| c.to_string().parse().unwrap())
                        .collect::<Vec<Octopus>>()
                })
                .collect(),
        ))
    }
}

impl HeatMap {
    fn first_step(&mut self) {
        self.0.iter_mut().for_each(|l| {
            l.iter_mut().for_each(|l| {
                l.energy += 1;
                l.flash = false;
            })
        })
    }

    fn flash_propagation(&mut self, row: isize, column: isize) {
        for (row_step, column_step) in SURROUND {
            let mut row_index = row + row_step;
            let mut column_index = column + column_step;
            if row_index < 0
                || row_index >= self.0.len() as isize
                || column_index < 0
                || column_index >= self.0[row_index as usize].len() as isize
            {
                continue;
            }

            if self.0[row_index as usize][column_index as usize].flash == false {
                self.0[row_index as usize][column_index as usize].energy += 1;
            }
        }
    }

    fn flash(&mut self) -> usize {
        let mut total: usize = 0;
        loop {
            let mut is_flashed: bool = false;
            for row_index in 0..self.0.len() {
                for column_index in 0..self.0[row_index].len() {
                    if self.0[row_index][column_index].energy > 9
                        && self.0[row_index][column_index].flash == false
                    {
                        self.0[row_index][column_index].energy = 0;
                        self.0[row_index][column_index].flash = true;
                        self.flash_propagation(row_index as isize, column_index as isize);
                        total += 1;
                        is_flashed = true;
                    }
                }
            }
            if is_flashed == false {
                break;
            }
        }

        total
    }
}

#[derive(Debug)]
struct Octopus {
    energy: usize,
    flash: bool,
}

impl FromStr for Octopus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            energy: s.trim().parse().unwrap(),
            flash: false,
        })
    }
}

impl HeatMap {
    fn print_heat_map(&self) {}
}

#[cfg(test)]
mod test {
    use crate::day11::HeatMap;
    use crate::utils::{self, get_lines, open_file_read};
    use std::io::{self, Read};
    use std::path::Path;
    use std::path::PathBuf;
    use std::str::FromStr;

    use super::Octopus;

    fn read_file(path: &str) -> Result<HeatMap, std::io::Error> {
        let mut reader = utils::open_file_read(&PathBuf::from_str(path).unwrap())?;
        let mut buff = String::new();
        reader.read_to_string(&mut buff)?;
        Ok(buff.parse().unwrap())
    }

    #[test]
    fn example1() -> std::io::Result<()> {
        let mut heatmap: HeatMap = read_file("data/day11_example")?;

        let mut total: usize = 0;
        for _ in 0..100 {
            heatmap.first_step();
            total += heatmap.flash();
        }

        assert_eq!(total, 1656);

        Ok(())
    }

    #[test]
    fn solution1() {
        let mut heatmap: HeatMap = read_file("data/day11").unwrap();

        let mut total: usize = 0;
        for _ in 0..100 {
            heatmap.first_step();
            total += heatmap.flash();
        }

        println!("{}", total);
    }

    #[test]
    fn example2() {
        let mut heatmap: HeatMap = read_file("data/day11_example").unwrap();

        let mut step: usize = 0;
        loop {
            step += 1;
            heatmap.first_step();
            if heatmap.flash() == 100 {
                break;
            }
        }

        println!("{}", step);
    }

    #[test]
    fn solution2() {
        let mut heatmap: HeatMap = read_file("data/day11").unwrap();

        let mut step: usize = 0;
        let max_size: usize = heatmap.0.iter().map(|l| l.len()).sum();
        loop {
            step += 1;
            heatmap.first_step();
            if heatmap.flash() == max_size {
                break;
            }
        }

        println!("{}", step);
    }
}
