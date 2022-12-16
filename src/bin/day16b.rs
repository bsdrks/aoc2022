#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::many_single_char_names)]

use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = "
Valve QE has flow rate=3; tunnels lead to valves OU, ME, UX, AX, TW
Valve TN has flow rate=16; tunnels lead to valves UW, CG, WB
Valve UX has flow rate=0; tunnels lead to valves AA, QE
Valve HK has flow rate=5; tunnels lead to valves HT, QU, TW, WV, OK
Valve SK has flow rate=14; tunnels lead to valves GH, GA, XM
Valve HY has flow rate=0; tunnels lead to valves LG, AA
Valve BK has flow rate=0; tunnels lead to valves SZ, AA
Valve BY has flow rate=11; tunnels lead to valves SP, HS, DN, KD, TK
Valve GR has flow rate=0; tunnels lead to valves FE, OK
Valve OH has flow rate=0; tunnels lead to valves BM, KE
Valve DC has flow rate=0; tunnels lead to valves AX, XH
Valve YS has flow rate=0; tunnels lead to valves XH, EU
Valve KP has flow rate=0; tunnels lead to valves KI, OF
Valve LG has flow rate=0; tunnels lead to valves FE, HY
Valve FE has flow rate=4; tunnels lead to valves RU, GR, YI, LG, ME
Valve NK has flow rate=0; tunnels lead to valves SD, BM
Valve EU has flow rate=0; tunnels lead to valves NS, YS
Valve OF has flow rate=0; tunnels lead to valves CJ, KP
Valve TW has flow rate=0; tunnels lead to valves HK, QE
Valve GL has flow rate=0; tunnels lead to valves AF, CQ
Valve OU has flow rate=0; tunnels lead to valves KN, QE
Valve BM has flow rate=24; tunnels lead to valves GH, NK, YH, OH
Valve GA has flow rate=0; tunnels lead to valves SK, SZ
Valve EI has flow rate=17; tunnels lead to valves ZX, AF
Valve QN has flow rate=25; tunnel leads to valve SD
Valve ZX has flow rate=0; tunnels lead to valves EI, WB
Valve ME has flow rate=0; tunnels lead to valves QE, FE
Valve CJ has flow rate=21; tunnels lead to valves OF, YI, KD
Valve AX has flow rate=0; tunnels lead to valves DC, QE
Valve LW has flow rate=0; tunnels lead to valves AA, HT
Valve CQ has flow rate=18; tunnels lead to valves GL, XM
Valve KN has flow rate=0; tunnels lead to valves SZ, OU
Valve HS has flow rate=0; tunnels lead to valves UZ, BY
Valve RU has flow rate=0; tunnels lead to valves TK, FE
Valve SZ has flow rate=6; tunnels lead to valves WV, GA, BK, KE, KN
Valve AF has flow rate=0; tunnels lead to valves GL, EI
Valve YI has flow rate=0; tunnels lead to valves FE, CJ
Valve HT has flow rate=0; tunnels lead to valves LW, HK
Valve WV has flow rate=0; tunnels lead to valves SZ, HK
Valve TK has flow rate=0; tunnels lead to valves BY, RU
Valve GH has flow rate=0; tunnels lead to valves BM, SK
Valve CG has flow rate=0; tunnels lead to valves TN, SP
Valve AA has flow rate=0; tunnels lead to valves HY, UX, VQ, LW, BK
Valve SP has flow rate=0; tunnels lead to valves BY, CG
Valve XM has flow rate=0; tunnels lead to valves SK, CQ
Valve DN has flow rate=0; tunnels lead to valves NS, BY
Valve XH has flow rate=22; tunnels lead to valves YS, QU, UZ, DC
Valve KI has flow rate=20; tunnels lead to valves UW, KP
Valve OK has flow rate=0; tunnels lead to valves HK, GR
Valve YH has flow rate=0; tunnels lead to valves VQ, BM
Valve UZ has flow rate=0; tunnels lead to valves XH, HS
Valve KE has flow rate=0; tunnels lead to valves OH, SZ
Valve VQ has flow rate=0; tunnels lead to valves AA, YH
Valve QU has flow rate=0; tunnels lead to valves HK, XH
Valve WB has flow rate=0; tunnels lead to valves TN, ZX
Valve UW has flow rate=0; tunnels lead to valves KI, TN
Valve SD has flow rate=0; tunnels lead to valves NK, QN
Valve NS has flow rate=23; tunnels lead to valves EU, DN
Valve KD has flow rate=0; tunnels lead to valves BY, CJ
";

/// The number of minutes during which the pressure should be maximized.
const LIMIT: usize = 30;

fn solve(input: &str) -> i64 {
    let mut pressure = HashMap::<String, i64>::new();
    let mut graph = HashMap::<String, Vec<String>>::new();

    input.lines().for_each(|l| {
        let tt = &mut l.split(' ');
        let name = tt.nth(1).unwrap();

        pressure.insert(
            name.to_owned(),
            tt.nth(2)
                .unwrap()
                .trim_start_matches("rate=")
                .trim_end_matches(';')
                .parse::<i64>()
                .unwrap(),
        );

        graph.insert(
            name.to_owned(),
            tt.skip(4)
                .map(|s| s.trim_end_matches(',').to_owned())
                .collect::<Vec<_>>(),
        );
    });

    // The relevant valves are the ones that have a non-zero flow rate.
    let relevant = pressure
        .iter()
        .filter(|(_, v)| **v != 0)
        .map(|(k, _)| k.clone())
        .collect::<Vec<_>>();

    let n = relevant.len();
    let m = 1 << n;

    // Precompute the distances between all pairs of relevant valves.
    let distances = graph
        .keys()
        .map(|v| (v.clone(), bfs(v, &graph)))
        .collect::<HashMap<_, _>>();

    // Previous valve and mask for each minute, valve, and mask.
    let mut previous = vec![vec![vec![None; m]; n]; LIMIT + 1];

    // Score per minute, valve, and mask.
    let mut dp = vec![vec![vec![i64::MIN; m]; n]; LIMIT + 1];

    for (i, dist) in relevant
        .iter()
        .map(|v| distances["AA"].get(v).unwrap())
        .enumerate()
    {
        dp[*dist + 1][i][1 << i] = 0;
    }

    for i in 1..=LIMIT {
        for j in 0..m {
            for k in 0..n {
                let flow = (0..n)
                    .filter_map(|i| ((1 << i) & j != 0).then_some(pressure[&relevant[i]]))
                    .sum::<i64>();

                let new = dp[i - 1][k][j] + flow;

                if new > dp[i][k][j] {
                    previous[i][k][j] = Some((i - 1, k, j));
                    dp[i][k][j] = new;
                }

                if ((1 << k) & j) == 0 {
                    continue;
                }

                for l in 0..n {
                    if ((1 << l) & j) != 0 {
                        continue;
                    }

                    let d = distances[&relevant[k]][&relevant[l]];
                    let next_minute = i + d + 1;

                    if next_minute > LIMIT {
                        continue;
                    }

                    let mask = j | (1 << l);
                    let new = dp[i][k][j] + flow * (d as i64 + 1);

                    if new > dp[next_minute][l][mask] {
                        dp[next_minute][l][mask] = new;
                        previous[next_minute][l][mask] = Some((i, k, j));
                    }
                }
            }
        }
    }

    let k = 1 << n;
    let v = &dp[26];

    (0..k)
        .flat_map(|i| {
            (0..k)
                .map(|j| {
                    let f = |i| v.iter().map(|v| v[i]).max().unwrap();

                    if (i & j) == j {
                        f(j) + f(i & !j).max(0)
                    } else {
                        i64::MIN
                    }
                })
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap()
}

fn bfs(start: &str, adj: &HashMap<String, Vec<String>>) -> HashMap<String, usize> {
    let mut distances: HashMap<String, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back(start.to_string());
    distances.insert(start.to_string(), 0);
    seen.insert(start.to_string());

    while let Some(node) = queue.pop_front() {
        for neighbor in &adj[&node] {
            if !seen.contains(neighbor) {
                seen.insert(neighbor.to_string());
                distances.insert(neighbor.to_string(), distances.get(&node).unwrap() + 1);
                queue.push_back(neighbor.to_string());
            }
        }
    }

    distances
}

fn main() {
    println!("{}", solve(INPUT.trim()));
}
