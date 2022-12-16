use std::{collections::HashSet, iter::repeat};

pub fn main() {
    let cmds = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

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
        .scan(((0, 0), (0, 0)), |((hx, hy), (tx, ty)), (dx, dy)| {
            *hx = *hx + dx;
            *hy = *hy + dy;
            if i32::abs_diff(*hx, *tx) > 1 || i32::abs_diff(*hy, *ty) > 1 {
                *tx = *hx - dx;
                *ty = *hy - dy;
            }
            Some(*tx * 1000 + *ty)
        })
        .collect::<HashSet<i32>>()
        .len();

    println!("{:?}", answ);
}
