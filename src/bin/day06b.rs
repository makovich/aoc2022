const N: usize = 14;

pub fn main() {
    // let buff = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    // let buff = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    // let buff = "nppdvjthqldpwncqszvftbrmjlhg";
    // let buff = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    // let buff = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let buff = include_str!("day06.input");

    let answ = buff
        .as_bytes()
        .windows(N)
        .enumerate()
        .map(|(i, v)| {
            // println!("{:?}", String::from_utf8_lossy(v));
            (
                i + 1 + N, /* enumerate() starts from 0, so 1 + N */
                v.iter().fold(0u32, |acc, chr| 1 << (chr % 26) | acc),
            )
        })
        .take_while(|(_, v)| v.count_ones() < N as u32)
        .last()
        .unwrap()
        .0;

    println!("{}", answ);
}
