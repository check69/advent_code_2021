use std::{
    collections::HashMap,
    fmt::{self, Display},
    iter::Map,
    ops::Range,
    str::FromStr,
    vec, hash,
};

#[derive(Debug, Clone)]
struct Signal {
    input: Vec<String>,
    output: Vec<String>,
    mapping: HashMap<usize, String>,
}

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2): (String, String) = sscanf::scanf!(s, "{} | {}", String, String).ok_or(())?;
        Ok(Signal {
            input: s1
                .trim()
                .split(' ')
                .map(String::from)
                .collect::<Vec<String>>(),
            output: s2
                .trim()
                .split(' ')
                .map(String::from)
                .collect::<Vec<String>>(),
            mapping: HashMap::with_capacity(10),
        })
    }
}

impl Signal {
    fn find_1_4_7_8_in_input(&self) -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        for string in self.input.iter() {
            if [2, 3, 4, 7].contains(&string.len()) {
                ret.push(string.clone());
            }
        }

        ret
    }

    fn find_numbers(&mut self) {
        let mut strings_with_5_letters: Vec<String> = vec![];
        let mut strings_with_6_letters: Vec<String> = vec![];
        for string in self.input.iter() {
            if string.len() == 5 {
                strings_with_5_letters.push(string.clone());
            }
            else if string.len() == 6 {
                strings_with_6_letters.push(string.clone());
            }
            else if string.len() == 2 {
                self.mapping.insert(1, string.clone());
            }
            else if string.len() == 3 {
                self.mapping.insert(7, string.clone());
            }
            else if string.len() == 4 {
                self.mapping.insert(4, string.clone());
            }
            else if string.len() == 7 {
                self.mapping.insert(8, string.clone());
            }
        }
        self.find_6_letters_numbers(strings_with_6_letters);
        self.find_5_letters_numbers(strings_with_5_letters);
    }

    fn find_5_letters_numbers(&mut self, strings: Vec<String>) {
        for string in strings {
            if self.mapping.get(&1).unwrap().chars().all(| c | string.contains(c)) {
                self.mapping.insert(3, string.clone());
            }
            else if string.chars().all(| c | self.mapping.get(&6).unwrap().contains(c)) {
                self.mapping.insert(5, string.clone());
            }
            else {
                self.mapping.insert(2, string.clone());
            }
        }
    }

    fn find_6_letters_numbers(&mut self, strings: Vec<String>) {
        for string in strings {
            if !self.mapping.get(&1).unwrap().chars().all(| c | string.contains(c)) {
                self.mapping.insert(6, string.clone());
            }
            else if !self.mapping.get(&4).unwrap().chars().all(| c | string.contains(c)) {
                self.mapping.insert(0, string.clone());
            }
            else {
                self.mapping.insert(9, string.clone());
            }
        }
    }

    fn get_output_number(&self) -> usize {
        let mut ret: usize = 0;
        let mut length: usize = self.output.len();

        for output in self.output.iter() {
            for (key, value) in self.mapping.iter() {
                if is_anagram(get_hashmap(value), &output) {
                    length -= 1;
                    ret += key * 10_usize.pow(length as u32);
                    break;
                }
            }
        }
        ret
    }
}

fn get_hashmap(s1: &String) -> HashMap<char, usize> {
    let mut letters: HashMap<char, usize> = HashMap::with_capacity(s1.chars().count());

    for letter in s1.chars() {
        if letters.contains_key(&letter) {
            *letters.get_mut(&letter).unwrap() += 1;
        }
        else {
            letters.insert(letter, 1);
        }
    }

    letters
}

fn is_anagram(mut letters: HashMap<char, usize>, s2: &String) -> bool {
    letters.len() == s2.len() && s2.chars().all(| character | letters.contains_key(&character))
}

#[cfg(test)]
mod test {
    use crate::day8::Signal;
    use crate::utils;
    use std::path::Path;

    use super::{get_hashmap, is_anagram};

    #[test]
    fn example1() {
        let data: Vec<Signal> = utils::read_data(Path::new("data/day8_example")).unwrap();
        let mut total = 0_usize;
        for signal in data {
            let mut strings = signal.find_1_4_7_8_in_input();
            for string in strings {
                let hash_map = get_hashmap(&string);
                for output in signal.output.iter() {
                    if is_anagram(hash_map.clone(), &output) {
                        total += 1;
                    }
                }
            }
        }
        println!("{}", total);
    }

    #[test]
    fn solution1() {
        let data: Vec<Signal> = utils::read_data(Path::new("data/day8")).unwrap();
        let mut total = 0_usize;
        for signal in data {
            let mut strings = signal.find_1_4_7_8_in_input();
            for string in strings {
                let hash_map = get_hashmap(&string);
                for output in signal.output.iter() {
                    if is_anagram(hash_map.clone(), &output) {
                        total += 1;
                    }
                }
            }
        }
        println!("{}", total);
    }

    #[test]
    fn example2() {
        let mut data: Vec<Signal> = utils::read_data(Path::new("data/day8_example")).unwrap();
        let mut total = 0_usize;
        for signal in data.iter_mut() {
            signal.find_numbers();
            total += signal.get_output_number();
        }

        println!("{}", total);
    }

    #[test]
    fn solution2() {
        let mut data: Vec<Signal> = utils::read_data(Path::new("data/day8")).unwrap();
        let mut total = 0_usize;
        for signal in data.iter_mut() {
            signal.find_numbers();
            total += signal.get_output_number();
        }

        println!("{}", total);
    }
}
