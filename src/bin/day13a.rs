pub fn main() {
    let pkts = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    let pkts = include_str!("day13.input");

    let sum = pkts
        .replace("10", "A")
        .split("\n\n")
        .filter_map(|v| v.split_once('\n'))
        .enumerate()
        .map(|(i, (l, r))| (i + 1, (l.as_bytes(), r.as_bytes())))
        .filter_map(|(i, (mut l, mut r))| loop {
            match (l[0], r[0]) {
                (a, b) if a == b => (l, r) = (&l[1..], &r[1..]),
                (_, b']') => break None,
                (b']', _) => break Some(i),
                (b'[', _) => (l, r) = (&l[1..], shift(r)),
                (_, b'[') => (l, r) = (shift(l), &r[1..]),
                (a, b) if a < b => break Some(i),
                _ => break None,
            }
        })
        .sum::<usize>();

    println!("{}", sum);
}

fn shift(s: &[u8]) -> &[u8] {
    let s = unsafe { std::slice::from_raw_parts_mut(s.as_ptr().sub(1) as *mut _, s.len() + 1) };
    s.swap(0, 1);
    s[1] = b']';
    s
}
