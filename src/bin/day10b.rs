#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

const INPUT: &str = "
addx 1
addx 5
noop
addx -1
noop
addx 3
addx 29
addx -1
addx -21
addx 5
noop
addx -20
addx 21
addx 2
addx 8
addx -1
noop
noop
noop
noop
addx 6
addx -1
addx -37
addx 40
addx -10
addx -25
addx 5
addx 2
addx 5
noop
noop
noop
addx 21
addx -20
addx 2
noop
addx 3
addx 2
addx -5
addx 12
addx 3
noop
addx 2
addx 3
addx -2
addx -37
addx 1
addx 5
addx 3
addx -2
addx 2
addx 29
addx -22
addx 13
noop
addx -8
addx -6
addx 7
addx 2
noop
addx 7
addx -2
addx 5
addx 2
addx -26
addx -11
noop
noop
addx 6
addx 1
addx 1
noop
addx 4
addx 5
noop
noop
addx -2
addx 3
noop
addx 2
addx 5
addx 2
addx -22
addx 27
addx -1
addx 1
addx 5
addx 2
noop
addx -39
addx 22
noop
addx -15
addx 3
addx -2
addx 2
addx -2
addx 9
addx 3
noop
addx 2
addx 3
addx -2
addx 2
noop
noop
noop
addx 5
addx -17
addx 24
addx -7
addx 8
addx -36
addx 2
addx 3
addx 33
addx -32
addx 4
addx 1
noop
addx 5
noop
noop
addx 20
addx -15
addx 4
noop
addx 1
noop
addx 4
addx 6
addx -30
addx 30
noop
noop
noop
noop
noop
";

fn main() {
    let mut pixel = 0_usize;
    let mut register = 1_i64;
    let mut screen = [['.'; 40]; 6];

    for l in INPUT.trim().lines() {
        let mut update_screen = |register: i64| {
            pixel += 1;

            let x = pixel % 40;

            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            if (((register - 1).max(0) as usize)..=(register + 1).max(0) as usize).contains(&x) {
                screen[pixel / 40][x] = '#';
            }
        };

        update_screen(register);

        if !l.starts_with("noop") {
            let v = l[5..].parse::<i64>().unwrap();

            register = (register + v) % 40;
            update_screen(register);
        }
    }

    println!(
        "{}",
        screen
            .iter()
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
