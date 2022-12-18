pub fn main() {
    let nstr = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    let nstr = include_str!("day10.input");

    let crt = "noop"
        .lines()
        .chain(nstr.lines())
        .map(|l| l.split_once(' ').unwrap_or(("noop", "")))
        .map(|(i, o)| match i {
            "addx" => vec![("addx", 0), ("addx", o.parse::<i32>().unwrap())],
            "noop" => vec![("noop", 0)],
            _ => unreachable!(),
        })
        .flatten()
        .scan(1, |r, (i, o)| {
            match i {
                "noop" => {}
                "addx" => {
                    *r += o;
                }
                _ => unreachable!(),
            }
            Some(*r)
        })
        .enumerate()
        .fold([' '; 240], |mut crt, (cyc, reg)| {
            let idx = cyc as i32 % 40;
            if [reg - 1, reg, reg + 1].contains(&idx) {
                crt[cyc] = '#';
            }
            crt
        });

    crt.as_slice().chunks(40).for_each(|line| {
        let line = String::from_iter(line);
        println!("{line}");
    });
}
