#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{collections::HashSet, str::FromStr};

pub const INPUT: &str = ":)";

pub const EX: &str = "
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cube {
    position: (i64, i64, i64),
    sides: [(i64, i64, i64); 6],
}

impl Cube {
    #[must_use]
    pub const fn new(position @ (x, y, z): (i64, i64, i64)) -> Self {
        let x = x * 2;
        let y = y * 2;
        let z = z * 2;

        Self {
            position,
            sides: [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ],
        }
    }
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tt = s.split(',').map(|s| s.parse::<i64>().unwrap());

        let x = tt.next().unwrap();
        let y = tt.next().unwrap();
        let z = tt.next().unwrap();

        Ok(Self::new((x, y, z)))
    }
}

fn solve(input: &str) -> usize {
    let cubes = input
        .lines()
        .map(|l| l.parse::<Cube>().unwrap())
        .collect::<HashSet<Cube>>();

    let sides = cubes
        .into_iter()
        .flat_map(|c| c.sides.into_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n = sides.len();

    n - (n - sides
        .iter()
        .copied()
        .collect::<HashSet<(i64, i64, i64)>>()
        .len())
        * 2
}

fn main() {
    println!("{}", solve(INPUT.trim()));
}
