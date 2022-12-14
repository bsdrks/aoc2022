#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

const INPUT: &str = "
A Z
A Z
A Z
B Z
C X
A Z
A Z
C Y
A Z
A Y
B Y
A Z
C X
A Z
A Z
A Z
A Z
A Y
A Z
A Z
C Y
C X
C X
C X
A Z
A Z
B Y
A Z
A Z
C Z
B Z
A Y
A Z
B Y
A Z
B Y
A X
B Z
A Z
A Z
A Z
C X
C X
A X
A Z
A Z
C X
A Z
B Y
A Z
B Z
A Z
A Z
B Z
B Z
C Y
B Z
A X
B Z
A Z
B Y
A Z
B Z
C X
A Z
B Y
B Z
A Z
B Y
C X
B Y
A Z
A Z
A Z
B Y
A Z
A Z
B Y
B Z
C Z
A X
A Z
A Z
C Z
C X
A Z
C X
A Z
A X
A Z
A Z
A Z
B X
B Y
A Z
A Z
A Z
C Y
B Y
C X
C X
A Z
A Z
A Z
A X
C X
B Z
C X
C X
A Z
A Z
A Z
A Z
B Z
A Y
A Y
C X
A Y
C X
B Y
A Z
C Z
A Z
B Y
A Z
C X
B X
A Y
C X
A Z
A Z
A Y
B Z
C X
C X
C X
B Y
A Z
A Z
B Y
A Z
B Z
C X
C Z
A Z
C X
B Z
A Z
B Y
C X
A Y
B Y
C X
C X
A Z
A Y
A Z
A Z
A Z
A Y
C X
A Z
A Z
B Z
A Z
B Z
A Z
C Z
C Z
A Z
A Z
B Z
A Z
B Z
A Z
B Z
B Y
B Y
B Z
B Y
A Z
A Z
A Z
A Z
A Z
A Z
A Z
A Z
C X
C X
A Z
A Z
C Y
A Z
C Y
B Y
C X
B Z
A Z
A Z
C X
A Z
B Y
A Z
A Z
A Z
B Z
A Z
B Y
A Z
B Y
A Z
A Z
C Z
A Z
C X
C Y
C X
C Y
C X
A Z
B Z
A Z
A Z
A Z
A Z
A Z
B Y
A Z
C X
C X
A Z
A Z
C Z
C Z
A Z
A Z
A Z
C X
A Z
A Z
A Z
A Y
A Z
C X
A Z
A Z
A Z
C X
C X
A Z
C Y
A X
A Z
A Z
A Y
B Z
A Z
A Z
A Z
C X
A Z
A Z
A Z
B Z
B Z
A Y
B Z
C X
B Y
A Z
A Z
A Z
A Z
A Z
A Z
B Z
C X
A Z
A Y
A Z
B Y
B Z
C Z
A Z
A Y
A Z
B Z
A Y
C X
A Z
A Y
A Z
C Y
A Z
A Z
B Y
C X
A Z
A Z
A X
A Z
C X
A Z
B Y
A Z
C X
A Z
B Y
A Z
C Y
B Z
C X
A Z
C X
B Y
A Z
C X
A Z
B Y
A Z
A Z
C Z
A Z
A Y
A Z
A Z
C X
A Y
A Z
B Z
A Z
A Y
C X
A Z
B Y
A Y
A Z
A Z
A Z
A Z
A Z
A Z
B Z
B Y
B X
C Y
B X
A Z
A Y
C X
A Z
A Z
C X
C Y
A Y
A Z
C X
B Y
B Y
C X
A Z
C Y
A Z
A Z
A Z
A Z
B X
A Z
A X
B Z
C Z
A Z
A Z
A Y
A Z
A Y
A Z
A Z
B Y
A Z
A Z
A Z
C X
A Z
A Z
A Z
B Z
A Z
A Z
A Z
A Z
A Z
B Z
A Z
A Z
C X
A Z
A Z
A Z
A Z
A Y
A Z
A Z
C Z
A Z
A Y
B Y
B Z
A Z
B Y
B Z
A Z
C X
A Z
B X
A Z
A Z
A Y
A Z
B X
B Z
A Z
B Z
A Y
A Z
A Z
A Z
B Y
A Z
B Z
A Z
B Z
A Z
A Y
A Z
B Z
A Z
C X
C X
A Z
A Z
B Z
A Z
A Z
C X
A Z
A X
A Z
A X
C X
A Y
C Y
B Y
A Z
A Y
A X
A Z
A Y
A Z
A Z
A Z
C X
B Z
A Z
B Y
A Z
A Y
A Z
A Z
B Z
C X
A Z
A Z
A Y
C X
C Y
B Z
A Z
A Z
B Z
C X
A Z
A Z
A Z
C Y
A Y
A Z
B Z
B Z
A Z
B Z
A X
A Z
B Z
A Z
A Z
B Y
A Z
A Y
B Y
A Z
B Z
C X
C Y
A Z
B Z
C X
A Z
B X
A Z
A Z
C X
A Z
A Y
B Z
B Y
A Z
A Y
C Y
A Z
C X
B Z
A X
A Z
A Z
A Z
A Z
A Z
A Z
A Z
A Y
A Z
A Z
C X
B Y
B Z
C X
C X
C X
A Z
A Z
B Z
B Y
A Z
A Z
B Y
B Y
A Y
A Y
A X
A Y
B Y
A Y
C X
A Z
A Z
A Z
A Y
B Y
A Z
C Y
B Z
C X
A Y
A Z
C Z
A Z
B Z
C X
B Z
C X
B Z
B Y
B Y
C X
A Z
A Z
B Z
B Z
B Z
A Z
A Z
A Z
A Z
A Z
C X
C X
C Y
B X
C X
A Z
A Z
A Z
A Z
A Z
A X
A Z
C X
A Z
C Z
A Z
C X
C Z
A Z
A Z
A Y
B Z
A X
A Z
A Z
A Z
C X
B Y
A Z
B Y
A Z
B Y
A Z
C X
A Z
C X
A Z
A Z
A Z
A Z
A Z
B X
B Y
C X
C X
A Z
C X
A Z
A Z
A Z
A Y
A Z
A Y
A Z
A Z
A Z
C Y
A Z
A Z
B Z
B Y
C X
A Y
A Z
B Z
A Z
A Z
A Z
B Y
C Z
A Y
C Y
A X
A X
B Z
A Y
C X
C X
B Z
A Z
B Z
A Z
A Z
A Y
B Z
C Y
A Z
A Z
A Z
C X
B Y
A X
A Z
C X
A Z
B Z
B Y
C X
B Y
A X
A Z
A Z
A Z
A Y
A Z
A Z
A Y
A Y
A Y
A X
A Z
A Z
B Z
A Y
A Z
C X
C X
A Z
A Z
B Z
B Y
A Z
A Z
C X
A Y
C X
B Z
A Z
A Z
A Z
A Y
A X
C Y
C X
C X
A Z
A Y
A Z
A Z
A Z
C X
A Z
A Z
B Z
C X
C X
B Z
B Z
C X
A Z
B X
A Z
B Z
A Y
A Z
A Z
C X
A Z
C Z
A Z
B Z
C X
A Z
B Y
A Y
A Z
A Z
B X
C X
B Z
C X
C X
C Y
B Z
C X
A Y
C X
A Z
A Z
A Z
A Y
A Z
C X
C Y
A Z
A Z
B Z
A Z
B Y
A X
A X
B Z
C X
B Y
A Z
C X
A Z
B Z
A Z
B Y
B Y
A X
B Y
A Z
A X
A Z
C X
A Z
A Z
B Y
C Y
A X
A Z
C Y
C X
A Z
A Z
A Z
A Y
A Z
C X
A Z
B Y
A Z
B Z
B Z
C X
B Y
A Z
C Y
A Y
A Z
A Z
C X
A Z
A Z
A Z
A Z
B Y
B Z
A Z
A Z
C Y
A Z
A Z
A Z
A Z
A Y
A Z
A Z
C Y
B Y
B Z
A Z
B Z
A Z
B Z
A Z
A Z
A Y
A Z
A Z
B Z
C Y
A Z
B Z
A X
C X
C X
A Z
A Z
A Z
A Z
A Z
C X
A Z
C X
A Z
B Y
A Z
C Y
A Y
B Z
C X
A Y
A Z
A Y
A Z
B Z
B Y
A Y
B Z
A Z
A Z
C Z
C Z
A Y
A Z
A Z
C X
A Z
A X
C Y
C X
A Z
A Z
B Y
B Z
C Y
A Z
B Z
A Z
C Z
A Z
A Y
B Y
C X
A Z
A Z
A Z
C X
A Z
A Y
A Z
A Z
B Z
C X
C X
C X
A Z
A Z
A Z
C X
A Z
A Z
A Z
A X
A Z
C X
C X
B Z
A Z
A Z
A Z
B Y
A Z
C X
A X
B Z
C X
A Z
C X
A Y
A Z
A Z
A Y
A Z
A Y
A X
C X
A Z
A Y
B Y
A Z
A Y
C Z
B Y
A Z
A Z
A Y
A Z
C X
A Z
B Y
B Z
B Z
A Z
A Z
A Z
C X
A Z
B Y
B Y
A Z
A Y
C X
A Z
C X
A Z
C X
C X
A Y
B Z
A Z
A Z
A Z
A Y
A X
B Y
A Z
B Z
B Y
C Y
C X
C Y
A Z
B Z
A Z
A Y
B X
C Z
A X
A Z
A Z
A Z
A Y
A Y
A Z
A Y
C X
A Z
A Y
C X
A Z
A Z
C Y
A Z
B Z
A Y
B Z
B X
B Z
C X
A Z
A Z
C X
A Z
A Z
C X
B Z
B Z
B Z
A Z
A Z
B Y
A Z
A Z
C Z
C Z
B Z
A Z
B Y
A Z
A Z
C X
B Z
A Z
A Z
A Z
C Y
B Y
C X
A Z
A Z
A Z
A Y
A Z
C X
A Z
C X
A Z
A Z
C Y
C X
C X
C X
C X
C X
B Z
B Z
B Y
B Y
C Z
A Z
B Z
A Z
A X
A Z
A Z
A Z
C X
A Z
C X
A Z
A Z
C X
C X
B Y
A Z
C Y
A Z
A Y
C Y
B Z
B Y
C X
A Z
C Y
A Z
A Z
A Z
B Y
C X
B Z
B Z
C X
B Y
B Z
A Z
C X
A Z
B Y
B Z
A Y
A Z
A Z
A Z
A Z
C Y
A Z
C Y
A Z
B Y
C X
B Z
A Z
A Z
B Z
C X
B Z
A Z
A Z
A X
B Y
A Z
A Z
A Z
A Z
A Y
B Z
A Z
A Z
A Z
B Z
C Y
C X
C X
B Z
A Z
B Z
C X
A Y
C X
B Y
B Z
C Y
B Y
B Y
A Y
C Y
A Z
A Z
B Z
B Z
A X
A Z
A Z
B Z
A Z
B Z
A X
A Z
A Y
A Y
A Z
B Z
A Z
B Z
A Z
B Z
C X
A Z
A Z
C X
A Z
A Z
A Z
C X
B Y
A X
B Z
B Z
C Y
A Z
A Z
C X
B Y
C X
A Z
A X
A Z
A Z
A Z
A Z
A Z
A Z
A Z
A Z
A Z
C X
B Z
B Z
C X
A Y
C Z
A Y
A Z
C Y
A Z
A Z
A Y
A Z
C X
C Z
A Z
A Y
A Z
A X
A Y
A Z
C X
B Z
A Z
B Z
C Y
C X
A Z
A Z
A X
A Z
B Z
C X
A Z
B Y
A Z
A X
A Z
B Z
A Z
A Z
C X
B Z
B X
A Z
A Z
C Z
A Z
A Z
A Y
B Z
A Z
C X
B Z
A Z
C Y
A Z
B Z
B Z
A Z
A Z
C X
B Z
C Y
A Y
A Y
B Z
A Z
A Z
C Z
B Z
A Y
A Y
C Z
C Z
A Z
B Z
B Y
A Z
B Y
A Z
A Z
A Z
C Y
A Y
A Z
A Z
A Z
A Z
C Z
A Z
B Y
A Z
B Z
C Y
A Z
B Y
B Z
A Z
A Y
A Z
A Z
B Z
A Z
A Z
A Z
C Z
B Z
C X
B Z
A Z
A Z
A Y
A Z
A Z
A Z
A Z
C Y
A Z
A Z
A Z
A X
A Y
B Y
C X
A Z
A Z
B Y
A Z
C X
A Y
A Z
A Z
C X
A X
A Z
A Z
A Z
B Y
B Z
A X
A Z
C X
A Z
C X
B Z
B Y
A Z
B Y
A Y
A Z
A Y
A Z
B Y
C X
B X
B Y
C X
A Z
A Z
A Y
A Z
A Y
B Z
B Y
B Y
A Y
A X
A Z
C X
A Z
B X
A Z
A Z
C X
B Y
B Z
B Z
B Y
B Z
A Y
C X
C Z
A Z
A Y
B Y
B Y
A Z
A Y
C Y
B Y
B Z
A Z
A Y
A Z
B Z
A Z
A Z
A Z
A Z
B Z
A Y
C X
A Z
A Z
B Y
A Z
B Y
B Z
A Z
C X
A Z
C X
B Y
A Z
B Z
C X
A Z
A Z
A Z
C X
C X
A Z
A Z
A Z
B Y
A Z
A Z
A Z
A Z
A Z
A Y
B Z
C Y
A Y
C X
A Z
A Z
B Y
B X
A Z
C Z
A Z
A Y
A Z
A X
A Z
A Z
A Z
C Z
C X
C X
B Y
C X
C X
B Y
C Z
C X
C X
A Y
A Z
C X
C Z
B Y
A Z
C X
C X
A Z
A Z
A Z
A Y
A Z
A Y
B Z
A Z
A Z
A Z
A Z
A Z
A Z
C X
C X
C X
C X
A Z
A Z
B Z
A Y
A Z
A Z
B Z
A Z
A Z
A Z
A Z
A Z
C X
C X
A Z
A Z
C X
C Z
C Z
A Z
A Z
B Y
B Z
A Z
A Z
B Y
B X
C X
B Y
C X
C X
A Z
C X
C X
C Y
C X
C X
C X
B Z
B Z
A Z
A Z
B Z
A Z
C X
A Z
B Y
A Y
A Z
C Z
B Y
B Z
B Z
A Z
C X
B Z
A X
B Y
A Y
B Y
B Z
A Z
A Z
A Z
A Z
A Z
A Z
C X
C X
B Z
A Z
B Y
A Z
B Z
A Z
B Y
A Z
A X
A Z
B Y
B Y
A Y
C X
A Z
A Z
C Y
A Z
A Z
B Z
C Z
B Z
A Z
A Z
C X
C Y
B Z
A Z
C X
A Z
C X
B Y
C X
B Z
A Z
A Z
C X
A Z
C X
A Y
C X
C X
C Y
B Z
C X
B Z
C X
A Z
A Z
C X
A Y
B Z
C X
B Z
C X
A Z
C X
C X
A Z
A Z
B Y
C Z
A X
A X
A Z
C X
A Z
A Z
A Z
A Y
A Z
A Z
A Z
A Z
B Z
C X
B Y
A Z
A Z
B Y
A Z
A Z
A Z
A Z
B Y
A Z
B Z
A Y
A Y
A Y
A Z
A Y
A Z
B Z
A Z
B Y
A Z
C X
A Y
C X
A Z
B Y
A Y
A Z
A Y
A Z
C Z
A Z
B Z
A Z
C X
B Z
A Z
A Z
C X
B Y
A Z
C X
C Y
A Z
B Y
A Z
B X
B Y
B Y
C Z
C X
A Z
C X
A Y
A Y
B Z
A Z
C X
B Z
A Z
A Z
A Y
C X
C X
A Z
A X
B Z
A Z
A Z
A X
A Z
C X
A Z
B Y
A Z
B X
B Y
A Y
C X
A Z
A X
A X
A Y
B Y
A Y
A Z
C X
B Z
A Z
A X
A X
A Z
C X
B Z
C X
B Y
A Z
C X
A Y
A Z
A Z
A Z
A Z
C Z
B Z
C Z
A Z
A Z
C X
B Y
A Y
B Z
C X
B Z
A Z
A Z
A Z
A X
B Y
C X
B Y
B Z
B Z
A Z
A Y
C Y
A Y
A X
A Z
C X
A Z
A Z
A X
A Z
A Z
C X
A Z
A X
A Z
C Y
A Z
B Z
A Y
A Z
C Z
A Y
A Y
B Z
A Y
A Y
A Y
A Z
A Z
A Z
A Z
C X
A Z
A Z
A Z
A Z
A Z
A Z
A X
A Z
A Z
A Z
A Z
A Z
A Y
B Z
A Y
A Z
C X
A X
B Z
A Z
A X
C Y
B Y
B Y
B Z
C X
C X
A Z
A Z
C X
C X
A Z
A X
A Z
A Z
A Z
A X
C X
C X
A Z
A Y
A Z
A Z
A Z
A Z
C Z
A Z
C X
B Z
C Z
C X
A Z
B Y
C X
C X
B Z
C X
B Y
A Y
B X
A Z
A Z
A Z
B Z
C X
A X
B Z
A X
A Z
C Z
B Z
A Z
A Z
C X
C X
A X
C X
C Y
A Z
A Z
A Z
B Z
C Z
A Z
A Z
A Z
A Z
A Z
B Z
A Z
A Z
C X
B Y
A Z
B Z
A Z
C X
A Z
A Z
A Z
B Z
C X
B Y
A Z
A Z
A Z
C Y
A Z
A Z
C X
C X
A Y
B Z
A Z
C Y
C X
A Z
A Z
A Z
C X
A Z
A Z
B Z
B Z
A Z
A Z
A Z
C X
C X
A Z
B Z
A X
B Y
C X
C X
C X
A Z
C X
A Z
A Z
B Y
A Y
B Z
B Z
A Z
A Z
C Z
A Y
A Y
A Z
A Z
C X
C Z
A Z
A Z
A Z
A Z
C Y
B Y
B Y
C Y
A Z
C X
B Z
A X
A Y
B Z
A Z
A Z
A Z
A X
A Z
A Z
A Z
A Y
C Z
A Y
C X
A Y
A X
C X
A Z
B Y
A Y
B Y
A Z
C X
C Z
A Z
C Z
A Y
A Z
A Z
B Z
B X
B Z
B Z
B Y
C Z
C X
A X
C Z
B Z
A Z
A Z
B Z
A Y
A Z
B Y
C X
A Z
B Z
C X
A X
B Z
A Z
A Z
C X
C X
C X
B Z
C Z
A Z
C X
A Z
A Z
A Z
A Z
C Z
C X
A X
B Y
A Z
C X
C Z
A Z
C X
B Z
B Y
A Z
C X
A Z
B Z
C Z
A Z
A Z
A X
B Y
C X
A Z
C X
A Z
A X
C X
A Z
C X
A Y
A Z
C X
C Z
C Y
B Z
B Z
A Y
B Z
A Z
A Z
A Y
A Z
C X
C X
A Z
A Z
C X
B Z
A Z
B Y
A Z
A Z
A Z
C X
A Z
A Y
B Y
A Z
C X
A Z
A X
A Z
A Z
C X
C Y
A Z
C X
A Z
A Y
C X
B Z
C Z
B Y
A Z
A Z
B Y
A Z
B Y
A Z
A Z
A Z
C X
A Y
C X
A Z
C Y
A Z
A Z
A Z
B Y
B Z
B Z
C Z
A X
A Z
A Z
C X
A Z
C X
A Z
A X
C Z
C X
A Z
A Z
A Z
A Z
B Z
A Z
A Z
C Y
A Y
A Z
C Y
C Y
A Z
B Z
A Z
B Z
A Z
B Z
C Y
C X
C X
A Y
A Z
B Z
C X
A X
A Z
A Z
A Y
A Z
A Z
A Z
C X
C X
A Y
A Z
A Z
C X
B Z
B Y
A Y
A Z
B Z
A Z
A Z
A X
A Z
A Z
B Y
A Z
A Y
B Z
C X
A Z
A Z
A Z
C X
A Z
B Z
B Z
A Z
A Z
A Z
A Z
B Z
A Z
A Z
A Z
A Z
C X
C X
A Z
B X
A Z
B Y
C Z
A Z
A Z
A Z
C Z
A Z
B Y
A X
A Z
C Z
A Z
C Z
A Z
C X
A Y
C X
C Z
A Y
A Z
A Z
C X
C X
A Z
A Z
A Z
B Z
B Z
A Y
B Y
C Z
B Z
B X
A Z
A Z
A Z
C X
B Y
A Z
B Z
C X
A Z
A Z
C X
C X
A Z
A Z
A Z
A Z
B Z
A Z
A Z
A Z
A Z
A Z
B Z
A Z
A Z
B Z
A Y
A Z
B Y
A Z
A Z
A Z
B Z
A Z
A Z
C X
B Y
C X
A Y
C X
A Z
A Z
A Z
C X
B Z
A Z
A Z
A Z
A Z
A Z
C X
A Z
A Z
A Z
A Z
A Z
A Y
C X
B Y
C X
A Z
A Z
A Z
A Z
A Z
B Y
A Z
C X
A Z
A Z
A Z
A Z
A Z
A Z
A Z
C X
A Z
C X
A Y
B Z
A Z
C X
A Z
C X
B Z
B Z
A Z
A Z
A Z
C X
A Z
B Z
B Y
C X
C X
C X
B Z
A X
A Y
A Z
A Z
A Z
A Y
A Z
A Z
A Y
A X
A Y
C X
A Y
C X
B Z
B Y
C X
B Y
B Y
C X
A Z
A Z
B Z
C X
A Z
A Z
B Y
A Z
B Y
A Z
A X
C Y
C X
B Z
C X
A Z
A Z
A Z
C X
A Z
C X
A X
A Y
A Z
A Z
C X
A Y
A Z
B X
A Y
A Z
A Z
A Z
A Z
A Z
A Z
C X
A Z
C X
A Z
C Y
A Z
A Z
B Z
A Y
A Z
B Y
B Y
A Z
C X
A Z
C X
C Z
A Z
B Z
B Z
A Z
A Z
C X
C X
B Y
A Z
A Z
B Z
C Z
B Y
B Y
C X
A Z
A Z
B Y
A Z
C X
C Z
A Z
B Z
C X
A Y
C X
A Z
B Y
B Y
A Y
A Z
A Z
A Z
A Z
A Z
A Y
A Z
A Y
A Z
B Y
B Y
B Y
C X
A Z
A Z
A Y
A Z
A Z
C X
A Y
C X
A Z
A Z
A Z
B Y
A Z
A Z
B Z
C X
A Z
B Y
B Y
C X
C Z
B Y
A Z
A Z
A Z
C X
A Z
B Z
C Z
C X
B Y
A Z
A Z
A Y
A X
A Z
A Z
A Z
A Z
A Z
A Y
B Y
A Z
B Z
B Z
A Z
C Y
A X
A Z
A Z
C X
C X
C Y
B Y
B Y
B X
A Z
A Z
B Z
A Z
";

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

fn solve(games: &[(Choice, Outcome)]) -> usize {
    let mut score = 0;

    for (a, b) in games {
        match (a, b) {
            (Choice::Rock, Outcome::Lose) => score += 3,
            (Choice::Paper, Outcome::Lose) => score += 1,
            (Choice::Scissors, Outcome::Lose) => score += 2,
            (Choice::Rock, Outcome::Draw) => score += 3 + 1,
            (Choice::Paper, Outcome::Draw) => score += 3 + 2,
            (Choice::Scissors, Outcome::Draw) => score += 3 + 3,
            (Choice::Rock, Outcome::Win) => score += 6 + 2,
            (Choice::Paper, Outcome::Win) => score += 6 + 3,
            (Choice::Scissors, Outcome::Win) => score += 6 + 1,
        }
    }

    score
}

fn main() {
    println!(
        "{}",
        solve(
            &INPUT
                .trim()
                .lines()
                .map(|line| {
                    let mut parts = line.split_whitespace();
                    let a = parts.next().unwrap().parse().unwrap();
                    let b = parts.next().unwrap().parse().unwrap();

                    (a, b)
                })
                .collect::<Vec<(Choice, Outcome)>>(),
        )
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let games = vec![
            (Choice::Rock, Outcome::Draw),
            (Choice::Paper, Outcome::Lose),
            (Choice::Scissors, Outcome::Win),
        ];

        assert_eq!(solve(&games), 4 + 1 + 7);
    }
}
