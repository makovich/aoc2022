use std::iter::repeat_with;

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

    let prod = pkts
        .replace("10", "A")
        .lines()
        .filter(|l| !l.is_empty())
        .zip(repeat_with(|| {
            Box::leak(String::from("[[6]]").into_boxed_str())
        }))
        .map(|(p, d)| (p.as_bytes(), d.as_bytes()))
        .filter_map(|(mut l, mut r)| loop {
            match (l[0] as char, r[0] as char) {
                (a, b) if a == b => (l, r) = (&l[1..], &r[1..]),
                (_, ']') => break None,
                (']', _) => break Some((1, 1)),
                ('[', _) => (l, r) = (&l[1..], shift(r)),
                (_, '[') => (l, r) = (shift(l), &r[1..]),
                (a, b) if a < b => break Some(((a < '2').into(), 1)),
                _ => break None,
            }
        })
        .fold([1, 2], |mut acc, (d1, d2)| {
            acc[0] += d1;
            acc[1] += d2;
            acc
        })
        .iter()
        .product::<usize>();

    println!("{:?}", prod);
}

fn shift(s: &[u8]) -> &[u8] {
    let s = unsafe { std::slice::from_raw_parts_mut(s.as_ptr().sub(1) as *mut _, s.len() + 1) };
    s.swap(0, 1);
    s[1] = b']';
    s
}
