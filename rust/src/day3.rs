use std::{str::FromStr, vec};

#[derive(Clone)]
struct Binary {
    numbers: Vec<usize>,
}

impl FromStr for Binary {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command: Vec<char> = s.chars().collect();
        let mut trial: Vec<usize> = vec![];
        for c in command {
            trial.push(c as usize - 0x30);
        }
        Ok(Binary { numbers: trial })
    }
}

impl Binary {
    fn to_number(&self) -> usize {
        let mut idx: u32 = 0;
        let mut ret: usize = 0;
        for number in self.numbers.iter().rev() {
            if *number == 1_usize {
                ret += 2_usize.pow(idx);
            }
            idx += 1;
        }
        ret
    }
}

fn find_all_occurrences_in_binary(data: &Vec<Binary>, index: usize) -> (Vec<Binary>, Vec<Binary>) {
    let mut ret1: Vec<Binary> = vec![];
    let mut ret0: Vec<Binary> = vec![];
    for binary in data {
        if binary.numbers[index] == 1 {
            ret1.push(binary.clone())
        } else {
            ret0.push(binary.clone())
        }
    }
    (ret1, ret0)
}

fn most_repeated(data: &Vec<Binary>) -> Binary {
    let (mut data1, mut data0) = find_all_occurrences_in_binary(&data, 0);
    for index in 1..data[0].numbers.len() {
        if data1.len() >= data0.len() {
            if data1.len() == 1 {
                return data1[0].clone();
            }
            let (d1, d0) = find_all_occurrences_in_binary(&data1, index);
            data1 = d1;
            data0 = d0;
        } else {
            if data0.len() == 1 {
                return data0[0].clone();
            }
            let (d1, d0) = find_all_occurrences_in_binary(&data0, index);
            data1 = d1;
            data0 = d0;
        }
    }
    data1[0].clone()
}

fn less_repeated(data: &Vec<Binary>) -> Binary {
    let (mut data1, mut data0) = find_all_occurrences_in_binary(&data, 0);
    for index in 1..data[0].numbers.len() {
        if data1.len() < data0.len() {
            if data1.len() == 1 {
                return data1[0].clone();
            }
            let (d1, d0) = find_all_occurrences_in_binary(&data1, index);
            data1 = d1;
            data0 = d0;
        } else {
            if data0.len() == 1 {
                return data0[0].clone();
            }
            let (d1, d0) = find_all_occurrences_in_binary(&data0, index);
            data1 = d1;
            data0 = d0;
        }
    }
    data0[0].clone()
}

#[cfg(test)]
mod test {
    use crate::{
        day3::{less_repeated, most_repeated},
        utils,
    };
    use std::path::Path;

    use super::{find_all_occurrences_in_binary, Binary};

    #[test]
    fn example1() {
        let data: Vec<Binary> = utils::read_data(Path::new("data/day3")).unwrap();
        // let data: Vec<Binary> = vec![
        //     Binary{numbers: vec![0, 0, 1, 0, 0]},
        //     Binary{numbers: vec![1,1,1,1,0]},
        //     Binary{numbers: vec![1,0,1,1,0]},
        //     Binary{numbers: vec![1,0,1,1,1]},
        //     Binary{numbers: vec![1,0,1,0,1]},
        //     Binary{numbers: vec![0,1,1,1,1]},
        //     Binary{numbers: vec![0,0,1,1,1]},
        //     Binary{numbers: vec![1,1,1,0,0]},
        //     Binary{numbers: vec![1,0,0,0,0]},
        //     Binary{numbers: vec![1,1,0,0,1]},
        //     Binary{numbers: vec![0,0,0,1,0]},
        //     Binary{numbers: vec![0,1,0,1,0]}
        //     ];
        let length = data.len() / 2;
        let mut sum = vec![0usize; data[0].numbers.len()];
        for d in data {
            let mut index = 0;
            for i in d.numbers {
                sum[index] += i;
                index += 1;
            }
        }

        println!("{:?}", sum);
        let mut number1: usize = 0;
        let mut number2: usize = 0;
        let mut idx: u32 = 0;
        sum.reverse();
        for total in sum {
            println!("total {} > {} length", total, length);
            if total > length {
                number1 += 2_usize.pow(idx);
                println!("{}", 2_usize.pow(idx));
            } else {
                number2 += 2_usize.pow(idx);
            }
            idx += 1;
        }
        println!("{} * {}", number1, number2);
        println!("{}", number1 * number2);
    }

    #[test]
    fn example2() {
        let data: Vec<Binary> = utils::read_data(Path::new("data/day3")).unwrap();
        // let data: Vec<Binary> = vec![
        //     Binary{numbers: vec![0, 0, 1, 0, 0]},
        //     Binary{numbers: vec![1,1,1,1,0]},
        //     Binary{numbers: vec![1,0,1,1,0]},
        //     Binary{numbers: vec![1,0,1,1,1]},
        //     Binary{numbers: vec![1,0,1,0,1]},
        //     Binary{numbers: vec![0,1,1,1,1]},
        //     Binary{numbers: vec![0,0,1,1,1]},
        //     Binary{numbers: vec![1,1,1,0,0]},
        //     Binary{numbers: vec![1,0,0,0,0]},
        //     Binary{numbers: vec![1,1,0,0,1]},
        //     Binary{numbers: vec![0,0,0,1,0]},
        //     Binary{numbers: vec![0,1,0,1,0]}
        //     ];

        let test1 = most_repeated(&data);
        let test2 = less_repeated(&data);
        println!("{}", test1.to_number() * test2.to_number());
    }

    #[test]
    fn example1_improve() {}

    #[test]
    fn example2_improve() {}
}
