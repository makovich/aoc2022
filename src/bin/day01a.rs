pub fn main() {
    let vs = include_str!("day01.input")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u16>>();

    dbg!(vs);
}
