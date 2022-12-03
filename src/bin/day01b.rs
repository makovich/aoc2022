pub fn main() {
    let mut sums = include_str!("day01.input")
        .lines()
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .fold(vec![0], |mut vec, num| {
            if num == 0 {
                vec.push(0);
            } else {
                vec.last_mut().map(|sum| *sum += num);
            }
            vec
        });

    sums.sort_unstable_by(|a, b| b.cmp(a));

    println!("{}", sums.iter().take(3).sum::<u32>());
}
