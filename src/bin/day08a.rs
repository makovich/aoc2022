#![allow(non_snake_case)]

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
    let mut lr = vec![Option::<u32>::None; N * N];
    let mut rl = vec![Option::<u32>::None; N * N];
    let mut tb = vec![Option::<u32>::None; N * N];
    let mut bt = vec![Option::<u32>::None; N * N];

    let f: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    // println!("{:?}", f);

    for y in 0..N {
        let (mut s_lr, mut s_rl, mut s_tb, mut s_bt) = (vec![], vec![], vec![], vec![]);
        for x in 0..N {
            update_lookup(&mut lr, &mut s_lr, &f, y * N + x);
            update_lookup(&mut rl, &mut s_rl, &f, (y + 1) * N - 1 - x);
            update_lookup(&mut tb, &mut s_tb, &f, x * N + y);
            update_lookup(&mut bt, &mut s_bt, &f, (N - 1 - x) * N + y);
        }
    }

    let mut vis = vec![];

    for i in 0..N * N {
        if [lr[i], rl[i], tb[i], bt[i]].iter().any(|v| v.is_none()) {
            vis.push(i);
        }
    }

    // println!("{:?}", lr);
    // println!("{:?}", rl);
    // println!("{:?}", tb);
    // println!("{:?}", bt);

    // println!("\n{:?}\n{}", vis, vis.len());

    println!("{}", vis.len());
}

/// https://www.geeksforgeeks.org/next-greater-element/
fn update_lookup(tbl: &mut [Option<u32>], stk: &mut Vec<usize>, forest: &[u32], idx: usize) {
    while stk.last().map_or(false, |&i| forest[i] <= forest[idx]) {
        stk.pop().map(|i| tbl[i] = Some(forest[idx]));
    }
    stk.push(idx);
}
