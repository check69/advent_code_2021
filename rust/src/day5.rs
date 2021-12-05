use std::{
    collections::HashMap,
    fmt::{self, Display},
    iter::Map,
    ops::Range,
    str::FromStr,
    vec,
};

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn get_horizontal_line(&self, end_point: usize) -> Vec<Coordinate> {
        let mut ret: Vec<Coordinate> = vec![];
        for x in self.x..end_point + 1 {
            ret.push(Coordinate { x: x, y: self.y });
        }

        ret
    }

    fn get_vertical_line(&self, end_point: usize) -> Vec<Coordinate> {
        let mut ret: Vec<Coordinate> = vec![];
        for y in self.y..end_point + 1 {
            ret.push(Coordinate { x: self.x, y: y });
        }

        ret
    }

    fn get_diagonal_line(&self, end_coordinate: &Coordinate) -> Vec<Coordinate> {
        let mut ret: Vec<Coordinate> = vec![];

        let mut step_x: i32 = -1;
        if self.x < end_coordinate.x {
            step_x = 1;
        }
        let mut step_y: i32 = -1;
        if self.y < end_coordinate.y {
            step_y = 1;
        }

        let mut curr_x = self.x;
        let mut curr_y = self.y;
        ret.push(self.clone());

        while curr_x != end_coordinate.x {
            curr_x = add(curr_x, step_x);
            curr_y = add(curr_y, step_y);
            ret.push(Coordinate {
                x: curr_x,
                y: curr_y,
            });
        }

        ret
    }
}

#[derive(Debug, Clone)]
struct Segment {
    start: Coordinate,
    end: Coordinate,
}

impl FromStr for Segment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x1, y1, x2, y2): (usize, usize, usize, usize) =
            sscanf::scanf!(s, "{},{} -> {},{}", usize, usize, usize, usize).ok_or(())?;
        Ok(Segment {
            start: Coordinate { x: x1, y: y1 },
            end: Coordinate { x: x2, y: y2 },
        })
    }
}

impl fmt::Display for Segment {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "start: x: {}, y: {} -> end: x: {}, y: {}",
            self.start.x, self.start.y, self.end.x, self.end.y
        )
    }
}

impl Segment {
    fn is_vertical_line(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_horizontal_line(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_vertical_or_horizontal_line(&self) -> bool {
        self.is_horizontal_line() || self.is_vertical_line()
    }

    fn get_coordinates(&self) -> Option<Vec<Coordinate>> {
        if self.is_horizontal_line() {
            if self.start.y > self.end.y {
                return Some(self.end.get_vertical_line(self.start.y));
            } else {
                return Some(self.start.get_vertical_line(self.end.y));
            }
        }

        if self.is_vertical_line() {
            if self.start.x > self.end.x {
                return Some(self.end.get_horizontal_line(self.start.x));
            } else {
                return Some(self.start.get_horizontal_line(self.end.x));
            }
        }

        Some(self.start.get_diagonal_line(&self.end))
    }
}

struct Overlap {
    coordinates: HashMap<Coordinate, usize>,
}

impl fmt::Display for Overlap {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        for (coordinate, value) in &self.coordinates {
            writeln!(f, "{}, {} = {}", coordinate.x, coordinate.y, value)?;
        }
        Ok(())
    }
}

impl Overlap {
    fn add_segment(&mut self, segment: &Segment) {
        if let Some(coordinates) = segment.get_coordinates() {
            for coordinate in coordinates {
                if self.coordinates.contains_key(&coordinate) {
                    *self.coordinates.get_mut(&coordinate).unwrap() += 1;
                } else {
                    self.coordinates.insert(coordinate, 1);
                }
            }
        }
    }

    fn count_coordinates(&self) -> usize {
        self.coordinates.values().filter(|v| **v > 1).count()
    }
}

#[cfg(test)]
mod test {
    use crate::{day5::Overlap, utils};
    use std::{collections::HashMap, path::Path};

    use super::Segment;

    #[test]
    fn example1() {
        let data: Vec<Segment> = utils::read_data(Path::new("data/day5")).unwrap();

        let mut overlap: Overlap = Overlap {
            coordinates: HashMap::new(),
        };
        for segment in data {
            if segment.is_vertical_or_horizontal_line() {
                overlap.add_segment(&segment);
            }
        }

        println!("{}", overlap.count_coordinates());
    }

    #[test]
    fn example2() {
        let data: Vec<Segment> = utils::read_data(Path::new("data/day5")).unwrap();

        let mut overlap: Overlap = Overlap {
            coordinates: HashMap::new(),
        };
        for segment in data {
            overlap.add_segment(&segment);
        }

        println!("{}", overlap.count_coordinates());
    }

    #[test]
    fn example1_improve() {}

    #[test]
    fn example2_improve() {}
}
