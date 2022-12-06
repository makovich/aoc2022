pub fn main() {
    // const N: usize = 3 + 1;
    // let (stks, cmds) = "    [D]
    // [N] [C]
    // [Z] [M] [P]
    // 1   2   3

    // move 1 from 2 to 1
    // move 3 from 1 to 3
    // move 2 from 2 to 1
    // move 1 from 1 to 2"
    const N: usize = 9 + 1;
    let (stks, cmds) = include_str!("day05.input").split_once("\n\n").unwrap();

    let mut stacks = vec![vec![]; N];

    stks.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| c.is_ascii_alphabetic())
            .for_each(|(i, c)| {
                stacks[i + 1].push(c);
            })
    });

    cmds.lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .step_by(2)
                .filter_map(|s| s.parse().ok())
                .enumerate()
                .fold([0usize; 3], |mut acc, (i, v)| {
                    acc[i] = v;
                    acc
                })
        })
        .for_each(|[n, from, to]| {
            for _ in 0..n {
                stacks[from].pop().map(|v| stacks[to].push(v));
            }
        });

    let answ = stacks.iter().skip(1).fold(String::new(), |mut acc, val| {
        acc.push(*val.last().unwrap());
        acc
    });

    println!("{}", answ);
}
