//'a means the life time of that variable will live. 'a it's a generic lifetime that its decided by the compiler.
fn count_increased_measures<'a>(
    measures: impl Iterator<Item = &'a usize> + Clone,
    skip: usize,
) -> usize {
    let iter1 = measures.clone();
    let iter2 = measures.skip(skip);
    iter1
        .zip(iter2)
        .map(|(a, b)| if a.lt(&b) { 1 } else { 0 })
        .sum()
}

fn is_increase(number1: &usize, number2: &usize) -> usize {
    if number2.gt(&number1) {
        return 1;
    }
    0
}

fn how_many_increase(numbers: Vec<usize>) -> usize {
    let mut prev_number = 0;
    let mut ret: usize = 0;
    for number in numbers {
        ret += is_increase(&prev_number, &number);
        prev_number = number;
    }

    ret - 1
}

#[cfg(test)]
mod test {
    use crate::utils;
    use std::path::Path;

    use super::{count_increased_measures, how_many_increase};

    #[test]
    fn example1() {
        let data: Vec<usize> = utils::read_data(Path::new("data/day1")).unwrap();
        let ret: usize = how_many_increase(data);
        println!("{}", ret)
    }

    #[test]
    fn example2() {
        let data: Vec<usize> = utils::read_data(Path::new("data/day1")).unwrap();
        let mut ret: usize = 0;
        let mut numbers = Vec::new();
        for number in data {
            if numbers.len() < 3 {
                numbers.push(number);
            } else {
                if number > numbers.remove(0) {
                    ret += 1;
                }
                numbers.push(number);
            }
        }
        println!("{}", ret)
    }

    #[test]
    fn example1_improve() {
        let data: Vec<usize> = utils::read_data(Path::new("data/day1")).unwrap();
        let result = count_increased_measures(data.iter(), 1);
        println!("{}", result);
    }

    #[test]
    fn example2_improve() {
        let data: Vec<usize> = utils::read_data(Path::new("data/day1")).unwrap();
        let result = count_increased_measures(data.iter(), 3);
        println!("{}", result);
    }
}
