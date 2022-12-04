pub fn main() {
    // let answ = "\
    // 2-4,6-8
    // 2-3,4-5
    // 5-7,7-9
    // 2-8,3-7
    // 6-6,4-6
    // 2-6,4-8"
    let answ = include_str!("day04.input")
        .lines()
        .map(|s| {
            s.split_once(',')
                .map(|(elf1, elf2)| {
                    let (a, b) = elf1.split_once('-').unwrap();
                    let (c, d) = elf2.split_once('-').unwrap();

                    (
                        a.parse::<u16>().unwrap(),
                        b.parse::<u16>().unwrap(),
                        c.parse::<u16>().unwrap(),
                        d.parse::<u16>().unwrap(),
                    )
                })
                .unwrap()
        })
        .fold(0, |acc, ranges| {
            acc + match ranges {
                (a, b, c, d) if a <= c && b >= d => 1,
                (a, b, c, d) if a >= c && b <= d => 1,
                _ => 0,
            }
        });

    println!("{:?}", answ);
}
