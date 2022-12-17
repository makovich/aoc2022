use std::{collections::HashSet, iter::repeat};

pub fn main() {
    // 1
    // let cmds = "\
    // R 4
    // U 4
    // L 3
    // D 1
    // R 4
    // D 1
    // L 5
    // R 2";

    // 36
    // let cmds = "\
    // R 5
    // U 8
    // L 8
    // D 3
    // R 17
    // D 10
    // L 25
    // U 20";

    let cmds = include_str!("day09.input");

    let answ = cmds
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(x, c)| (x.chars().nth(0).unwrap(), c.parse().unwrap_or(0)))
        .map(|(x, c)| repeat(x).take(c))
        .flatten()
        .map(|c| match c {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => unreachable!(),
        })
        .scan([(0, 0); 10], |rope, (dx, dy)| {
            rope[0].0 += dx;
            rope[0].1 += dy;

            rope.iter_mut()
                .reduce(|h, t| {
                    let (hx, hy) = h;
                    let (tx, ty) = t;
                    if i32::abs_diff(*hx, *tx) > 1 || i32::abs_diff(*hy, *ty) > 1 {
                        let (dx, dy) = (*tx - *hx, *ty - *hy);
                        let dist = i32::max(dx.abs(), dy.abs());
                        let (xx, yy) = (dx / dist, dy / dist);
                        *tx = *hx + xx;
                        *ty = *hy + yy;
                    }
                    t
                })
                .map(|(x, y)| *x * 1000 + *y)
        })
        .collect::<HashSet<i32>>()
        .len();

    println!("{:?}", answ);
}
