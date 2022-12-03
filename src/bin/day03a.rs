pub fn main() {
    // let answ = "\
    // vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw"
    let answ = include_str!("day03.input")
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .fold(0, |acc, (a, b)| {
            let mut map_a = 0u64;
            let mut map_b = 0u64;

            for (ca, cb) in a.chars().zip(b.chars()) {
                let ca = ca as u64 - if ca.is_lowercase() { 96 } else { 38 };
                let cb = cb as u64 - if cb.is_lowercase() { 96 } else { 38 };
                map_a |= 1 << ca;
                map_b |= 1 << cb;
            }

            acc + (map_a & map_b).trailing_zeros()
        });

    println!("{:?}", answ);
}
