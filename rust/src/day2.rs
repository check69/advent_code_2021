

fn parse_command(command: String) -> (String, i32) {
    let mut iter = command.split(" ");
    (iter.next().unwrap().to_string(), iter.next().unwrap().trim().parse().unwrap())
}

#[cfg(test)]
mod test {
    use crate::utils;
    use std::path::Path;

    use super::parse_command;

    #[test]
    fn example1() {
        let data: Vec<String> = utils::read_data(Path::new("data/day2")).unwrap();
        let mut depth = 0;
        let mut horizontal = 0;

        for line in data {
            let (command, number) = parse_command(line);
            match command.as_str() {
                "forward" => { horizontal += number; }
                "down" => { depth += number; }
                "up" => {depth -= number;}
                _ => {}

            }
        }
        println!("{}", horizontal * depth);
    }

    #[test]
    fn example2() {
        let data: Vec<String> = utils::read_data(Path::new("data/day2")).unwrap();
        let mut depth = 0;
        let mut horizontal = 0;
        let mut aim = 0;

        for line in data {
            let (command, number) = parse_command(line);
            match command.as_str() {
                "forward" => {
                    horizontal += number;
                    depth += aim * number;
                }
                "down" => {
                    aim += number;
                }
                "up" => {aim -= number;}
                _ => {}

            }
        }
        println!("{}", horizontal * depth);
    }

    #[test]
    fn example1_improve() {
    }

    #[test]
    fn example2_improve() {
    }
}
