#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

const INPUT: &str = "
Monkey 0:
  Starting items: 54, 53
  Operation: new = old * 3
  Test: divisible by 2
    If true: throw to monkey 2
    If false: throw to monkey 6

Monkey 1:
  Starting items: 95, 88, 75, 81, 91, 67, 65, 84
  Operation: new = old * 11
  Test: divisible by 7
    If true: throw to monkey 3
    If false: throw to monkey 4

Monkey 2:
  Starting items: 76, 81, 50, 93, 96, 81, 83
  Operation: new = old + 6
  Test: divisible by 3
    If true: throw to monkey 5
    If false: throw to monkey 1

Monkey 3:
  Starting items: 83, 85, 85, 63
  Operation: new = old + 4
  Test: divisible by 11
    If true: throw to monkey 7
    If false: throw to monkey 4

Monkey 4:
  Starting items: 85, 52, 64
  Operation: new = old + 8
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 7

Monkey 5:
  Starting items: 57
  Operation: new = old + 2
  Test: divisible by 5
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 6:
  Starting items: 60, 95, 76, 66, 91
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 2
    If false: throw to monkey 5

Monkey 7:
  Starting items: 65, 84, 76, 72, 79, 65
  Operation: new = old + 5
  Test: divisible by 19
    If true: throw to monkey 6
    If false: throw to monkey 0
";

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Number(usize),
    Old,
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "old" {
            Self::Old
        } else {
            Self::Number(s.parse().map_err(|_| ())?)
        })
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    operator: Operator,
    operand: Operand,
}

impl Operation {
    #[must_use]
    pub const fn evaluate(&self, value: usize) -> usize {
        let rhs = match self.operand {
            Operand::Number(n) => n,
            Operand::Old => value,
        };

        match self.operator {
            Operator::Add => rhs + value,
            Operator::Multiply => rhs * value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisor: usize,
    targets: (usize, usize),
    inspections: usize,
}

impl Monkey {
    pub fn inspect(&mut self, i: usize) -> (usize, usize) {
        self.inspections += 1;
        self.items[i] = self.operation.evaluate(self.items[i]);

        let target = if self.items[i] % self.divisor == 0 {
            self.targets.0
        } else {
            self.targets.1
        };

        (target, self.items.remove(i))
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ll = s.lines().map(str::trim);

        let items = ll.nth(1).unwrap()[15..]
            .split(", ")
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let l = ll.next().unwrap();
        let operator = l[21..22].parse().unwrap();
        let operand = l[22..].trim().parse().unwrap();
        let divisor = ll.next().unwrap()[19..].trim().parse().unwrap();

        let targets = (
            ll.next().unwrap()[25..].trim().parse().unwrap(),
            ll.next().unwrap()[25..].trim().parse().unwrap(),
        );

        Ok(Self {
            items,
            inspections: 0,
            operation: Operation { operator, operand },
            divisor,
            targets,
        })
    }
}

const ROUNDS: usize = 10000;

fn solve(s: &str) -> usize {
    let mut monkeys = s
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<Monkey>>();

    let prime_product = monkeys.iter().map(|m| m.divisor).product::<usize>();

    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let (target, item) = monkeys[i].inspect(0);

                monkeys[target].items.push(item % prime_product);
            }
        }
    }

    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();

    inspections.sort_unstable();
    inspections.reverse();

    inspections[0..2].iter().product()
}

fn main() {
    println!("{}", solve(INPUT.trim()));
}
