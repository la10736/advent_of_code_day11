#![feature(conservative_impl_trait)]

use std::io::prelude::*;

fn read_all<S: AsRef<std::path::Path>>(path: S) -> String {
    let mut content = String::new();
    let mut f = std::fs::File::open(path).unwrap();
    f.read_to_string(&mut content).unwrap();
    content
}

fn main() {
    let fname = std::env::args().nth(1).unwrap_or(String::from("example"));
    let content = read_all(fname);

    let d = directions(&content);

    let hops = Coord(0,0).steps(&d).hops();
    let max_hops = Coord(0,0).path(&d)
        .map(|c| c.hops()).max().unwrap();

    println!("Hops = {}", hops);
    println!("Max Hops = {}", max_hops);
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
struct Coord(i32, i32);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    N,
    NW,
    NE,
    S,
    SW,
    SE
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "n" => Direction::N,
            "nw" => Direction::NW,
            "ne" => Direction::NE,
            "s" => Direction::S,
            "sw" => Direction::SW,
            "se" => Direction::SE,
            d => Err(format!("Cannot understand direction '{}'", d))?
        })
    }
}

fn directions<S: AsRef<str>>(data: S) -> Vec<Direction> {
    data.as_ref().split(',').map(|t| t.parse().unwrap()).collect()
}

impl Coord {
    fn step(self, d: Direction) -> Self {
        use Direction::*;
        match d {
            N => Coord(self.0, self.1 + 2),
            NW => Coord(self.0 - 1 , self.1 + 1),
            NE => Coord(self.0 + 1, self.1 + 1),
            S => Coord(self.0, self.1 - 2),
            SW => Coord(self.0 - 1, self.1 - 1),
            SE => Coord(self.0 + 1, self.1 - 1)
        }
    }

    fn steps<I: AsRef<[Direction]>>(self, directions: I) -> Self {
        self.path(directions.as_ref()).last().unwrap_or_default()
    }

    fn path<'a>(mut self, directions: &'a [Direction]) -> impl Iterator<Item=Coord> + 'a {
        directions.as_ref().iter().map(move |d| {self = self.step(*d); self})
    }

    fn hops(&self) -> i32 {
        let m = self.0.abs().min(self.1.abs());
        m + (self.1.abs() - m)/2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Direction::*;

    #[test]
    fn move_rules() {
        assert_eq!(Coord(0, 2), Coord(0, 0).step(N));
        assert_eq!(Coord(-1, 1), Coord(0, 0).step(NW));
        assert_eq!(Coord(1, 1), Coord(0, 0).step(NE));
        assert_eq!(Coord(0, -2), Coord(0, 0).step(S));
        assert_eq!(Coord(-1, -1), Coord(0, 0).step(SW));
        assert_eq!(Coord(1, -1), Coord(0, 0).step(SE));
    }

    #[test]
    fn apply_directions() {
        assert_eq!(Coord(3, 3), Coord(0, 0).steps(vec![NE, NE, NE]));
        assert_eq!(Coord(0, 0), Coord(0, 0).steps(vec![NW, NW, SE, SE]));
        assert_eq!(Coord(2, -2), Coord(0, 0).steps(vec![NE, NE, S, S]));
    }

    #[test]
    fn path() {
        let dirs = vec![NE, NE, NE];
        assert_eq!(vec![Coord(1, 1), Coord(2, 2), Coord(3, 3)],
                   Coord(0, 0).path(&dirs).collect::<Vec<_>>());
    }

    #[test]
    fn compute_hops_distance() {
        assert_eq!(0, Coord(0, 0).hops());
        assert_eq!(3, Coord(3, 3).hops());
        assert_eq!(3, Coord(2, 4).hops());
        assert_eq!(4, Coord(2, 6).hops());
    }

    #[test]
    fn read_input() {
        assert_eq!(N, "n".parse().unwrap());
        assert_eq!(NW, "nw".parse().unwrap());
        assert_eq!(NE, "ne".parse().unwrap());
        assert_eq!(S, "s".parse().unwrap());
        assert_eq!(SW, "sw".parse().unwrap());
        assert_eq!(SE, "se".parse().unwrap());
    }

    #[test]
    fn integration() {
        assert_eq!(3, Coord(0, 0).steps(
            "se,sw,se,sw,sw".split(',')
                .map(|t| t.parse::<Direction>().unwrap()).collect::<Vec<_>>()).hops())
    }
}
