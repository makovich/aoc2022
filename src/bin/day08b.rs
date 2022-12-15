#![allow(non_snake_case)]

#[derive(Copy, Clone)]
enum Axis {
    LR = 0,
    RL = 1,
    TB = 2,
    BT = 3,
}

use Axis::*;

/// O(n) solution from redditor https://www.reddit.com/user/depressionpear/
/// https://painted-brush-a63.notion.site/day-8-O-n-with-next-greater-element-algorithm-ce3fb66663be44aea022ba1731fbe33b
pub fn main() {
    let input = "\
30373
25512
65332
33549
35390";

    let input = include_str!("day08.input");

    let N = input.lines().nth(1).map(|l| l.len() as usize).unwrap();

    // lookup tables
    let mut tbl = vec![vec![None; N * N]; 4];

    // stacks
    let mut stk = [
        vec![0; N * N],
        vec![0; N * N],
        vec![0; N * N],
        vec![0; N * N],
    ];

    let f: &[u32] = &input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    let update = |x, t: &mut [Vec<Option<usize>>], s: &mut [Vec<usize>], i| {
        let s = &mut s[x as usize];
        let t = &mut t[x as usize];

        while s.last().map_or(false, |&j| f[j] <= f[i]) {
            let j = s.pop().unwrap();

            let dist = j.max(i) - j.min(i);

            t[j] = (dist > 0).then_some(match x {
                LR | RL => dist % N,
                TB | BT => dist / N,
            });
        }

        s.push(i);
    };

    // build lookup tables
    for y in 0..N {
        stk.iter_mut().for_each(|s| s.clear());
        for x in 0..N {
            update(LR, &mut tbl, &mut stk, y * N + x);
            update(RL, &mut tbl, &mut stk, (y + 1) * N - 1 - x);
            update(TB, &mut tbl, &mut stk, x * N + y);
            update(BT, &mut tbl, &mut stk, (N - 1 - x) * N + y);
        }
    }

    let [r, l, d, u] = &tbl[..] else {
        unreachable!();
    };

    let answ = (0..N * N)
        .map(|i| {
            r[i].unwrap_or(N - 1 - i % N)
                * l[i].unwrap_or(i % N)
                * d[i].unwrap_or(N - 1 - i / N)
                * u[i].unwrap_or(i / N)
        })
        .max();

    println!("{}", answ.unwrap());
}
