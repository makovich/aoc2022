use std::{cell::Cell, collections::VecDeque};

const NEIB: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn main() {
    let hmap = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
        .to_owned();

    let hmap = include_str!("day12.input").to_owned();

    let (m, s, e) = hmap
        .lines()
        .fold((vec![], (0, 0), (0, 0)), |(mut acc, mut s, mut e), val| {
            val.find('S').map(|v| s = (v, acc.len()));
            val.find('E').map(|v| e = (v, acc.len()));

            acc.push(unsafe { std::mem::transmute::<&[u8], &[Cell<u8>]>(val.as_bytes()) });

            (acc, s, e)
        });

    m[s.1][s.0].set(b'a');
    m[e.1][e.0].set(b'z');

    let res = bfs(m.as_slice(), s, e).unwrap();

    println!("{res}");
}

fn bfs(map: &[&[Cell<u8>]], s: (usize, usize), e: (usize, usize)) -> Option<u32> {
    let mut q = VecDeque::from([(e, 0)]);

    while let Some(((x, y), len)) = q.pop_front() {
        if s == (x, y) {
            return Some(len);
        }

        let cur = rm_mark(&map[y][x]);

        NEIB.iter()
            .filter_map(|(dx, dy)| {
                let x = (x as isize + dx) as usize;
                let y = (y as isize + dy) as usize;

                match map.get(y).and_then(|row| row.get(x)) {
                    Some(s) if seen(s) => None,
                    Some(s) if s.get() >= cur - 1 => Some((x, y)),
                    _ => None,
                }
            })
            .for_each(|(x, y)| {
                mark(&map[y][x]);
                q.push_back(((x, y), len + 1));
            });
    }

    None
}

#[inline]
fn seen(square: &Cell<u8>) -> bool {
    square.get() & 1 << 7 != 0
}

#[inline]
fn mark(square: &Cell<u8>) {
    square.set(square.get() | 1 << 7);
}

#[inline]
fn rm_mark(square: &Cell<u8>) -> u8 {
    square.get() & !(1 << 7)
}
