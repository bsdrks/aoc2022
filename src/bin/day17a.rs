#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{collections::HashSet, ops::Add};

const INPUT: &str = ">>><<<>>>><<<>>>><<<<>>><<<>><<<<><<<<>><<<<><<<><>>>><<<>>><<<<>>><>>>><<<>>>><>>><>>><<<<>>>><<>>><>>><>>>><<<<>>>><<<>>><>><<<>>><<<>>>><<<>>><>>>><<>>>><<<<>>><<>>>><<<>>>><>>><<<>><<>>>><<>>>><><<<>><<<>>>><><>>>><<><<<<>>>><<<>>>><<<><>>><<<>>>><<<>>><<<>><<<<>><<<<><<<>><<><>>><<<<>>>><<<<>><<>>>><<<<>><<<<>><<<<><<<>><<<>><<<>><<>><<>>><<<<>>><<<<>><<<><<<>><>>>><<<<>>><><<<<>><<<<><<><<<>>>><<<<>><>><<<<>>><<<>>>><<<>>><<<><<><<<>>>><<<<><><<<>>><<<<>>>><<<<>>><<<>>>><<<<><<<>>><<>><<<><>>>><<<<>>>><<<>><<<>>><<<><<<><<<<>>>><<>>>><<<>>>><><>>>><<>>>><<<>><>>>><<<<>>><<<><>>><<<><<<<>>><<<><<<>>><>>><<<>>>><>>><<<>><><>><<<<>><>>><>>>><<>><<<>><<>>><<>>>><<<<>>>><<<<>>>><<<>>><<>>>><<><<>>><<>>><<<<>>><<>><>>>><><><<<<>>>><<<<>><<<<><<<<>>>><<<>>><<><<<>>>><<><><<>>><<>><<<>>><>><<<>><<<<>>><><<>><<<>>><<<<>><>>><<>><>>><<<><>><<>>>><>>><<>>><<<<>><<<<>>>><>>><<<><<<<>>><<>>><>>><<<<>>>><<<<>><<<>>>><<>>><>>><>>><<<>><<<<>>><<<<>>>><>>><>>>><<<<><<><<<<>>>><>><>>>><<<><<<>>><>>><<<<><>>><<<>><<<>>><<<>>>><>>><>>>><<>><<>>><<<<>>>><><<<<>><<<>><<>>><>><><<>><<>>>><<<<>>>><<<>>><<>><>>><<<<><<<<>><<>>>><<<<>>>><<<>><><<<<>>>><><<<><<<<>>>><>>>><<<>>><<<<>>>><<<>>>><<>>>><<><<>><<<<>><<>>>><<<><<>><<<<>>><<<>>><<>>><<<>>>><<>>>><<>>><<<<>>><<<>>>><<>>>><<<<>>><<<>><<<>><<><<<<>><>><>><<<><<>>>><<<<><>>>><<<<><<>>><<>>>><<>>><>>><><<><<<<>>>><<>>>><<<>><<>>>><<<>>><<<>>><<<><<<<>><<<>><<<<><<>>><<<>>><<<>><<<>>>><<><>><<<><<<<>><<>><<<>><<<<>><<<>>>><>><<>>>><<<><<>>>><<<>>>><>>><<<>>>><<<<>>><<<><<>>><<<>>><<>><<>>>><<<<>>><<<><<<><>><>>><<<<><<<<><<<<>><<<<>>><<<<><<><<>>><<<<>><<<>><><>>>><>><<<<>><<>>>><><>>><><<>>><>><<<<>><>>><>><<<<><<<>>><>><<<>>>><><>>><<<>>>><<><><<<<>>><<>><<<>><<><<<<>>>><<>><>>><<>>><<>>>><<<>>><<>>>><<<<>>>><<<<>><<<<>>>><<<<>>><<<><<<>>><>>>><>>>><<<><<<<>>><><<<<>>>><>><<>>>><<<<>>><<>>><><<<<>>><<<<>>>><<<<>>>><<><>>>><<><<>>><<><<<<>>>><<<<>><<<><>>>><>>>><<>>><<<<><><<>>><<>>><><><><>>><<><<>>><>>><<<<>>>><><<<>>>><>>><<<>>><<<>>>><<<>>>><<>>><>>>><<>><<<>>><<<>><<<<><<>>>><<<<>>><<<<>>><>>><<<<>><<<<>><<<<>><<<><<<>>>><<<>>>><>>><<>><<<<><<<>>>><<<>>><<<<>>>><>><<<><<>>>><<>>><<<<>>><>>><<<>><<<<>>>><<<>><<<>>>><>>>><<<><<<<>>>><>>>><<<<>>>><<<<><<>>>><<>>><>>>><<<<>>><>>><>><<<>>><>>>><<<<>><<>>><<<>>>><<>>><<><<<<><<<>><<><<>><<<><<><<<<>><<<<><<><<>>><<<>><<>>>><<>>><<>>>><<<<>>><<<>>><<>>>><<>>>><<<<>>><><>>>><<>>>><<>><<>>><<<>><<<>><>>><<<>><<<>><<<>>><<<<><<<<>>><<<<>>>><<<>>><<>><<>>><<<><<><<>><<<<>><<><<<<>><<<<>>>><>>>><><<<>>>><<<<>>><>><<<>>>><<>>>><>>><<<>><>>>><>><>>>><<<<>>><<><><<<>>><<<<>>>><<<><>><>><<<>>>><>>>><>>>><><<<<>><<<<><><<<>>>><<>>><<<<>>><<<>>>><>>><<><<>>>><<>><<>><<<<>>>><<>>>><><<<<><<><<<<>><<>><<<<><<<><<<<>>>><<<>><<<<>><<<<>>>><<>>><<<>><>>><>>><<>><>>>><><<>><<<><<>>>><<>>><<>>><<>>><<<<>>>><<>>><>><>>><<<><<<>><<>>>><<<<>>>><<<<>>>><<>>><<<>>>><<<<>><<<<>>>><>>>><>>><>>>><<<<>><><<><<<<>>><<<>>><<>>>><>><><<<>>>><<<<>>>><<>><<>><<<><<<<><<<<>><<<<>>>><>>>><>>>><<>>><<<>>><<<>>>><<>><<<>><<<><>>><<>>>><<<><>>>><><<<<><<>><><>><>><<<>><<>>>><<>>><<<><<>>>><<<<>>>><>>>><<<<>>><<<<>>><<><<<<>><>>>><<>>>><>>>><<<>>><>><<<>><<><<<<>><<<>>>><<<<>><<<<>>><<<>>><<<>>>><<<<>>><<<>><<<<>>><<<>>>><>>>><<><>><<>>>><<<<>>><><<<<>>><><>>>><>><<<<>>><<<<>>>><<><<<>>><>>>><<<>><<>>>><<<<>><><<>>><<>>>><>><<<><<>>><<<>>><<<<>><><<><><<<<>><<<<>><<>>><<<>>>><<<<>><<<>><<<>>><<><<<<>>><<<>>><>><>>><>><>><<<<>>>><>><<<<>><<>>>><>>>><>>>><<<<>>><<<<><<>>><>>><<<<>>>><<<>>>><<>>>><<<><<<><<<<><>>>><<<>><<<>>>><<<<>><<>>><<<>>>><<<><<<>>><<><><<>>><<<>><<<<><<<<>>><<><<<>><><>>>><<>><<<<><<<<>><<<><<><<<<>>>><<>>>><<<<>><<<>>><<><>>>><<>><>><<<<>><<<>>>><>><<<<>>><><<<><<>>><>>>><<<<>>><<<><>>><>>>><<<<>>><<>><<>>><<<<><<<<>><<><<><<>><>>><<<>>>><>>><>>>><<<<><<>><>>><<<>>><>>>><<<<>>>><<>>><>>><<<<>>>><<>>>><<<>>><<<>>>><<<>><>>><<>>><<<><<<<>>>><<<>>><<>><<<<>><>>>><<>>><<<<>><<<>>>><<>>>><<>><><<<<>>><<<<>><<<>><<<<>>>><<>>><<><<>>><<>><>><>>>><<<<><<>>><<<<><<<<>><<><<>>><<<>>><<<<>><<<<>>><>><<<<>>><<<>>>><>>><>><<<>>><<>>>><<<<>>>><<>>><<<<>><<><>>><<>>><<<<>>><<<<><<>><<<<><<<>>>><<>>>><>><<<><<<<>>>><<<<>><<<<><<>>>><<<>>>><>>><>><<<<><<<<>>>><<<>>><<<>>>><><>>><<<<><<<<>>>><<<>>><<>>>><<><>><<><<<<>>>><>>>><<<>><<<>><<<<>>><<<<>><<<>><>>><<<>>>><<<<>><<<<>>>><<>>><<<>>>><<<<>>>><<<>><>><<>><><<<<>><<><><<>><<>><<<<>>><<>>>><<<<>>>><<><<<<>>>><<<>><<<<>>>><<<>>><<><>>>><<<>>><>>>><<<<>>>><>><<><<<<>>><<>>><<<<>><<<>>>><<>>>><<<><<<>>>><<<<><<<>><<<<>>><<<>><<<><<<>><>>><<<<>>><<<>>><<<>><<<>>>><<<<>>><<<<>>>><<<<>><<<<><<<>>><><<<>><<<>><<<>><<<>>><<<><<<>>>><<<>><<<<>>>><><<<<>><<<<>>><<<<>><<><<<>>>><>>><><<<<>>><>>><<<<>>>><<<<>>><<<>>>><><<>><><<><<<<><<<<>>><>><>>>><<<<><<<<>><<<><>>>><<>>><<><<>>><<>>>><<<<>>>><<><>>>><<<<><<>><<><<>><<<<>>><<<<>><<<<>>>><>><<><<<>>>><<<>><<>>>><<<><<<<>>>><<<>>><<>><<<>>>><<>><<<<>>>><<<<>><<<>>>><>><<<<><<<>>>><<<><<<>><>>><<>>>><<<<>>><<<><<><>><<><>><<<><><<<><<<<>>><<>><<<><<>><<<><<<>><<<>>><>>>><<<>>>><<<<>>>><<<>><>><<>>><<<><>>><<<>><<<><<>><>>>><><>>>><<<>>><<<>>>><>>><<<<>>><>>>><<<>>>><<>>><<>>>><<>><><>>>><<<>>>><>>><>><<>>><<>>>><>><<>>><<><>>>><<<>><<>>><><<<>><<>>>><<<<>>>><>>><<<<>><>>><<>><<><<<>>><>><<<><<>><<>>><<><><<<<>>>><<<>>><>>>><>><<<>>><<<>>><<<<>>>><><<<<>>><<<<><<<>><<<<>>>><>>>><<>><<<<>>>><<>>><>><<><<<>>><<<>>>><<<<>>><<<>>><>>><<<>>><<<<>><<<><<>>>><><<><<>><>><<<<>>>><>><<>><<<<>>><<>>>><<<>><<<>>><<<>>><<<<>>><<<<>>>><<<<>><>>>><<<<>>>><>>>><>>><<<<><<<<>>><><<>>><<>>>><<<<>><<<<>>><<<>><<<>>>><<<<>>><<<>>>><<<><>>>><>>><<>>>><<<><<<><>>>><<<>>>><<>>>><>>>><<><<<<>><<<<><>>>><<><<<<>>>><>>><<<<>>><<>>>><<<><<<<><><<>>><<<>>><<>><<<>>>><>>>><<<<>>>><<<>>><<<>><<><<<>>>><<<<>><<<>>><>>><>><<<>>>><<<<>>>><<><<<>>>><>>>><<<>>>><<>><<<<>>><<<>>><><><<<<>>>><<<>>>><><<>><<>>>><<<<>>>><<<><<>>>><>>><<<<>><<>><<<>><><<<<>>><><<<>>>><<<><<<>>><<<<>><<<<>>>><<<>>>><<>><<<>>><<<<>>><><<>><<<<>>>><<<<>>><<>>><<<<><>><>>><<<<><>>>><<>><>><<<>>>><<<>>><<>><<><>>>><<<>>>><<<<>>>><<>>>><<<<>>>><<<>>>><<<>><<<>>><>>><>>><>>>><<>>>><<<>>><<<<>>><<>><<<><<<<>><>>><<<>>><<<>>><><<>>><>>>><<<<>>><<>>>><<<>>>><<<<>>><<<<>>>><<<>>><<><<>>><<>>>><<<>>><<><<<>>>><><<<<><><<><<>><>>>><<>>><><<><<<<>>><<<>>><<<<>>>><>><<<<>><<<<>><<><<<>>><>>>><>>>><<<<>>>><<<<><><<<><>>><<<<>><>><><<<<>>><<<><<><<<<>><<><>>>><>>>><<<>>>><<><<>>><<>><<<<>>><<<<>>>><>>>><<<<>><<>>><<<>>>><<<>>>><<<>>>><<<<>>><<<><<<><>>><<<><>>><<<><>>><>><<<>>><<<>>><<<<>>>><<>><<<>><>><<<<>>><<<>>>><<><<<>>>><<<<><><>>>><<>><>>>><<>>><<>>>><<>>>><>>><>>><<<>>><<>>><>><>>>><<<>>><>><>>>><<<>>><<><<<>>>><<<>><<><<<>>><<<<>>><<<<><<<<>>>><<<>><<<<>>><<<<>>><<>><<<<><<<><<<<>>>><<<<><>>>><>>>><<><<<<>><<>><<>>><>>><>>>><<<>>>><>><<<<>>>><<<<>><<<<>><<<>><<<<><>>>><>><<<>>><>><>>><>>><<<<>>><<>><<<>>><<<<>>><<><<<>>><<<>>>><<>>><<<<>><<<<>>><<><<>>><<<<>>><<<>>><<<<>>><<>>><<<>><<<>>>><<<>><<<<>>>><<>><>>>><<>>><>><<<>><<<<>>>><<>><<<<><<<>>>><<<>>><<>><<<>>>><<>>><<<<>><<>><<>><<<<>>>><<<>>><<<<>>>><<<>>><>>>><<<>>>><<>><<<<>><<>>>><<<<><<<>><<<<><<>>>><<<<>>>><<>>><<<<><<<>>>><<<>><<><<<<><><<>>>><><<<<><<<><<<<>><<><<<<><<<<>><<<>><<>><<<<><<>>><<><<<>><<>>><<<>>>><<>><<>><<<<>>><<<<>><<>>>><<<>>><<<<>><<>>><<><><<><<>>>><>><>>><<<><<<<>><<><<>>><<<<><<>>><>>>><<<><<><<>>>><><>>>><<>><<><>>><<><<<<>><<>>>><<>>><<<>>>><<<><>>><<>>>><>><<><<>><<<><<<>><<>><>><><><<>>>><<<>>><>>><<<<>>><<<>>><<<>><>>><<<<>><><<<><>>>><<>>><>>><<<>>><<<>><>>>><<>>><<<><<<>>>><><<<<>>><<<<>><<>><<<><<<<>><<<>>>><<<><>><<<<><<<<>>><<<>>>><<<<><<<>>><>>>><<<<>>><<><<>>>><<><<>><<>><>><<>>>><<<<>>><>>>><<>>>><<><<<>>><>>><<<>>>><<<<>><<<<><<<><<<><<<><<<>>>><<><<><><>><>>><<<>><>>><<<><>>><<>>><<<<><<<>>><<<<>>><<<<>>><<>>>><<<>>>><<<>><<<<><<<<>><<<><<<><<<>><<>>>><<<>><<<><>>><>>><<><<<>>><<<<>><<<>>>><>><<<>><>>><><<<><<<<><<<><<<>>><<><<>>>><<>>><><<<<>>>><<<>>>><<>><<<>><<>>><<><<<>><<>><<>><<<<>><<<<>><<<>>>><>>>><<>>><<<<><<>><<<>><<>>>><<<<>>><>><>><<<>>>><>>><<<>>><<>><<<<>><<><<<><<<<>><<<<><>><<<>>>><<<>>>><<>>><>><<>>>><<<>><<<>>>><><<<><<>><>><<>><<>>>><<>>><<<<><<><>>><><<<>>><<>>><<<>>><<<>>>><>>><<<<>>>><<<>>><<>><<<>>><<<>>><>>>><<><<<<>>>><<<><<<<>>><<<<>>><>><<<>>>><<><<><<<>>>><><<<>><<<<><<<>>><>>><>><<><<>><<<<>>>><>>><><<<<>>><<>><<<>>><><<<>>>><<<<>>><<>><<<><<><<<<>>>><<<>>>><<<>>>><><>><<<>>>><<><>>>><<<<>>>><<<>>>><<>>><<<<>>>><>>><>>><<>>>><<<><<>>>><><<<<>><<<>>>><<><<<<>>>><<>>>><><<<>>>><>><<<>>>><<>>><<>>><><<<>>><<<>><><<<<>>><<>>>><>>><<<>>><<>>>><<>><<<<><<>><<<>><<<<>>><>><<<<>>><<<>>>><>>>><<<>>>><>>><<<<>>><>><<<<><<<>><>>><>>>><<>>>><<>><<<>><<>>><<<>><<<<><<>><<<<>><<<<>><<<><>>>><>><><<<><<><>>><<<<><<<>><<>>><>>><<<<>><<<><<<>><>>>><<<<><><<>><<<<>><<<<>><>>>><><<<><>>><<<<>>>><<<>>><<>>>><<<>><<<>><>><<<><<<<>>><<<>>>><<<<>>>><<<>><<<<>>><>><<<<>><><<<<>><><<<<>><<>><<<<>>><>>>><<<<>>>><>>><>><>>>><<<>>>><><<<><<<>><<<<>>><<<<>><<<>>><<<<><<<>>><<<>>><><>>>><<<><<<<><<<>><<<<>>><<><<<<>><>>><<<<>>><>><>>>><><<<>>>><<><>>>><<><<<>>><<>>><<<<><>>><<><<>>><<<>>><><<<>>><>>><<<<>>>><<<>>>><<>>><<<<>>>><<>><<<<>><<<<><<>><<<<><<<>>>><<<><>>>><<>>>><<<>>>><<<>><<<>>><<>>>><>>><<<<>><<<<>><>>><<<<>><<>>><<<<><<<<>><<<<>>><<>><<><<<>><<>>>><<<<>><<<<>>><>>>><<>><>>>><<<><<<<>>><>>>><<<>><<<<>><<>>><<<>><<><>><<><>>>><>>>><<<<><>>><>><<<<>>>><<<>>>><<<>>>><<<>>>><<<><<<>>><<<>><<<<><<<><<>><<<>>>><<<>>>><<<<><>>><<><<<<>>>><>><<>>>><<>>>><>>><<<>>>><<><<<<>>>><>><><<<<>><>>><>>>><<<>>><<<<>>><<<<><<<>>>><>><<<><>>><<<<>>><>><>>><<<>><>><<><<<>><>>>><<><<<>>>><><<<>>>><<>><<>>>><>><><<<<><<<>>>><<<><<<>><<<<>><<><<>><>>>><<<<>>>><>><<<<>>>><<<<><>>>><<<><<<<><>><<>><<<>>><>><<<<>>><><<<>>><>><<>><<>>>><<<<>>><<<>>>><<<<><>><<<><<<><<><<<><<<<>>>><><><<><<>>><<>>><<>><>>><>>>><<><<<><>>><<<<>><<><<>><<<<>><<>>>><<<>>>><>>><<<>>><<<>>><<<>><>><<<<>>>><<>>>><<<>>>><>><<<>>>><<<>>>><<<>>><<>><<>><<<<><<><>><><<>><<>>>><<<>><>>><><<><<<><<>><<>>><<<>>>><<><>>>><<<<><<>>><><<<<>><<<>><<<<><<><<>>><>><>>>><<>>>><<<>>><<>>>><<<<>>>><<<>>>><>>>><>>><<<><>><<><>><<<<>>><<<<>>>><<<<>>>><>><><<<>>>><<<<>><<>>>><<<<><><<<>>>";

pub const EX: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Jet {
    Left,
    Right,
}

impl TryFrom<char> for Jet {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

/// A position in two-dimensional space.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    /// Left to right.
    x: i64,
    /// Depth.
    y: i64,
}

macro_rules! point {
    ($x:expr, $y:expr) => {
        Point { x: $x, y: $y }
    };
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Rocks.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rock {
    Minus,
    Plus,
    L,
    Pipe,
    Square,
}

impl Rock {
    #[must_use]
    pub fn points(&self) -> HashSet<Point> {
        match self {
            Self::Minus => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Self::L => vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
            Self::Pipe => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Self::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
        .iter()
        .map(|(x, y)| point!(*x, *y))
        .collect()
    }

    #[must_use]
    pub fn index(index: usize) -> Self {
        match index % 5 {
            0 => Self::Minus,
            1 => Self::Plus,
            2 => Self::L,
            3 => Self::Pipe,
            4 => Self::Square,
            _ => unreachable!(),
        }
    }
}

const MAX_ROCKS: usize = 2022;
const WIDTH: i64 = 7;
const MAX_X: i64 = WIDTH - 1;

/// A simulation of rocks falling into a pit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Simulation {
    /// The number of rocks dropped so far.
    dropped: usize,
    /// The jets.
    jets: Vec<Jet>,
    /// The current position of the rock being dropped.
    pos: Point,
    /// Solidified rocks.
    solid: HashSet<Point>,
    /// The height of the pile.
    height: i64,
    /// The number of turns since the start of the simulation.
    turn: usize,
}

impl Simulation {
    #[must_use]
    pub fn new(jets: Vec<Jet>) -> Self {
        Self {
            dropped: 0,
            jets,
            pos: point!(2, 3),
            solid: (0..WIDTH).map(|x| point!(x, -1)).collect(),
            height: 0,
            turn: 0,
        }
    }

    fn free(&self, rock: Rock, offset: Point) -> bool {
        rock.points().iter().any(|&p| {
            let p = p + self.pos + offset;

            self.solid.contains(&p) || p.y < 0 || p.x < 0 || p.x > MAX_X
        })
    }

    pub fn drop(&mut self) -> i64 {
        loop {
            let jet = self.turn % self.jets.len();
            let rock = Rock::index(self.dropped);

            match self.jets.get(jet) {
                Some(Jet::Left) if !self.free(rock, point!(-1, 0)) => {
                    self.pos.x -= 1;
                }
                Some(Jet::Right) if !self.free(rock, point!(1, 0)) => {
                    self.pos.x += 1;
                }
                _ => {}
            };

            self.turn += 1;

            if self.free(rock, point!(0, -1)) {
                self.dropped += 1;

                for p in rock.points() {
                    let p = p + self.pos;

                    self.height = self.height.max(p.y + 1);
                    self.solid.insert(p);
                }

                self.pos = point!(2, self.height + 3);

                return self.height;
            }

            self.pos.y -= 1;
        }
    }
}

fn solve(input: &str) -> i64 {
    let mut simulation = Simulation::new(
        input
            .chars()
            .map(|c| Jet::try_from(c).map_or_else(|_| panic!(), |direction| direction))
            .collect::<Vec<Jet>>(),
    );

    while simulation.dropped < MAX_ROCKS {
        simulation.drop();
    }

    simulation.height
}

fn main() {
    println!("{}", solve(INPUT.trim()));
}
