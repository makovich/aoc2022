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
    //     .split_once("\n\n")
    //     .unwrap();
    const N: usize = 9 + 1;
    let (stks, cmds) = include_str!("day05.input").split_once("\n\n").unwrap();

    let mut stacks = stks
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .fold([None; N], |mut acc, (i, c)| {
                    acc[i] = c.is_ascii_alphabetic().then_some(c);
                    acc
                })
        })
        .fold(vec![vec![]; N], |mut acc, line| {
            line.as_slice()
                .iter()
                .enumerate()
                .filter(|(_, c)| c.is_some())
                .for_each(|(i, c)| {
                    acc[i + 1].push(c.unwrap());
                });
            acc
        });

    cmds.lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .step_by(2)
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<usize>>()
        })
        .for_each(|cmd| {
            let mut tmp = vec![];

            (0..cmd[0]).for_each(|_| {
                stacks[cmd[1]].pop().map(|v| tmp.push(v));
            });

            tmp.into_iter().rev().for_each(|v| stacks[cmd[2]].push(v));
        });

    let answ = stacks.iter().skip(1).fold(String::new(), |mut acc, val| {
        acc.push(*val.last().unwrap());
        acc
    });

    println!("{}", answ);
}
