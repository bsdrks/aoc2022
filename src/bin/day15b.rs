#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

const INPUT: &str = "
Sensor at x=1384790, y=3850432: closest beacon is at x=2674241, y=4192888
Sensor at x=2825953, y=288046: closest beacon is at x=2154954, y=-342775
Sensor at x=3553843, y=2822363: closest beacon is at x=3444765, y=2347460
Sensor at x=2495377, y=3130491: closest beacon is at x=2761496, y=2831113
Sensor at x=1329263, y=1778185: closest beacon is at x=2729595, y=2000000
Sensor at x=2882039, y=2206085: closest beacon is at x=2729595, y=2000000
Sensor at x=3903141, y=2510440: closest beacon is at x=4006219, y=3011198
Sensor at x=3403454, y=3996578: closest beacon is at x=3754119, y=4475047
Sensor at x=3630476, y=1048796: closest beacon is at x=3444765, y=2347460
Sensor at x=16252, y=2089672: closest beacon is at x=-276514, y=2995794
Sensor at x=428672, y=1150723: closest beacon is at x=-281319, y=668868
Sensor at x=2939101, y=3624676: closest beacon is at x=2674241, y=4192888
Sensor at x=3166958, y=2890076: closest beacon is at x=2761496, y=2831113
Sensor at x=3758241, y=3546895: closest beacon is at x=4006219, y=3011198
Sensor at x=218942, y=3011070: closest beacon is at x=-276514, y=2995794
Sensor at x=52656, y=3484635: closest beacon is at x=-276514, y=2995794
Sensor at x=2057106, y=405314: closest beacon is at x=2154954, y=-342775
Sensor at x=1966905, y=2495701: closest beacon is at x=2761496, y=2831113
Sensor at x=511976, y=2696731: closest beacon is at x=-276514, y=2995794
Sensor at x=3094465, y=2478570: closest beacon is at x=3444765, y=2347460
Sensor at x=806671, y=228252: closest beacon is at x=-281319, y=668868
Sensor at x=3011731, y=1976307: closest beacon is at x=2729595, y=2000000
";

const MAX: i64 = 4_000_000;

fn solve(input: &str) -> i64 {
    let pairs = input
        .lines()
        .map(|l| {
            let mut pairs = l
                .trim_start_matches("Sensor at x=")
                .split(": closest beacon is at x=")
                .map(|s| {
                    let mut tt = s.split(", y=").map(|s| s.trim().parse::<i64>().unwrap());

                    (tt.next().unwrap(), tt.next().unwrap())
                });

            (pairs.next().unwrap(), pairs.next().unwrap())
        })
        .collect::<Vec<_>>();

    for &((x1, y1), (x2, y2)) in &pairs {
        let distance = (x1 - x2).abs() + (y1 - y2).abs();

        for (dx, dy) in [(-1, -1), (1, -1), (1, 1), (-1, 1)] {
            for d in 0..distance {
                let x_next = x1 + dx * d;
                let y_next = y1 + dy * (distance + 1 - d);

                if !(0..=MAX).contains(&x_next) || !(0..=MAX).contains(&y_next) {
                    break;
                }

                if pairs.iter().all(|&((x, y), (dx, dy))| {
                    (x_next - x).abs() + (y_next - y).abs() >= (x - dx).abs() + (y - dy).abs()
                }) {
                    return x_next * MAX + y_next;
                }
            }
        }
    }

    unreachable!()
}

fn main() {
    println!("{}", solve(INPUT.trim()));
}
