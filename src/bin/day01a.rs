pub fn main() {
    let max = include_str!("day01.input")
        .lines()
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .fold((0, 0), |(max, cur), num| {
            if num == 0 {
                (max.max(cur), 0)
            } else {
                (max, cur + num)
            }
        });

    println!("{}", max.0);
}
