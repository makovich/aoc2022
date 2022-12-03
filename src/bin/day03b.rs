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
        .scan((3, u64::MAX), |(elf, map), line| {
            if *elf == 0 {
                *elf = 3;
                *map = u64::MAX;
            }

            *elf -= 1;

            let mut map_cur = 0;
            for c in line.chars() {
                let c = c as u64 - if c.is_lowercase() { 96 } else { 38 };
                map_cur |= 1 << c;
            }

            *map &= map_cur;

            // println!("{:064b}", map);
            // println!("{:064b}", map_cur);
            // println!("{:064b}", x);
            // println!();

            Some(*map)
        })
        .skip(2)
        .step_by(3)
        .fold(0, |acc, map| acc + map.trailing_zeros());

    println!("{:?}", answ);
}
