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

    // let hmap = include_str!("day12.input").to_owned();

    let (m, s, e) = hmap
        .lines()
        .fold((vec![], (0, 0), (0, 0)), |(mut acc, mut s, mut e), val| {
            val.find('S').map(|v| s = (v, acc.len()));
            val.find('E').map(|v| e = (v, acc.len()));

            acc.push(unsafe { std::mem::transmute::<&[u8], &[Cell<u8>]>(val.as_bytes()) });
            // acc.push(unsafe { &*(val.as_bytes() as *const _ as *const [Cell<u8>]) });
            // acc.push(val.as_bytes());
            (acc, s, e)
        });

    m[s.1][s.0].set(b'a' - 1);
    m[e.1][e.0].set(b'z' + 1);

    println!("{m:?}\n\ns={s:?} e={e:?}");

    // let a: () = m;

    // with_flip(&Cell::new(b'a'), || {});

    // let res = with_flip(&m[s.1][s.0], s.0 as isize, s.1 as isize, || {
    // walk(s.1 as isize, s.0 as isize, b'a', 0, m.as_slice())
    // });

    // let res = walk(s.1 as isize, s.0 as isize, b'a', 0, m.as_slice());

    let res = bfs(m.as_slice(), s, e);

    println!("{res:?}");
}

fn bfs(map: &[&[Cell<u8>]], s: (usize, usize), e: (usize, usize)) -> Option<u32> {
    let mut q = VecDeque::from([(e, 0)]);
    let mut l = 0;

    while let Some(((x, y), len)) = q.pop_front() {
        if s == (x, y) {
            println!("{l:?}");
            return Some(len);
        }

        // if seen(&map[y][x]) {
        //     println!("seen");
        //     continue;
        // }

        // let cur = if e == (x, y) {
        //     b'z' + 1
        // } else {
        //     map[y][x].get()
        // };

        let cur = map[y][x].get() << 1 >> 1;
        let ch = char::from(cur);

        println!("= {ch} {:08b}", cur);

        NEIB.iter()
            .filter_map(|(dx, dy)| {
                // asdf
                let x = (x as isize + dx) as usize;
                let y = (y as isize + dy) as usize;

                // dbg!((x, y));
                match map.get(y).and_then(|row| row.get(x)) {
                    Some(s) if seen(s) => None,
                    // Some(s) if s.get() <= cur + 1 => Some((x, y)),
                    // Some(s) if s.get() == cur + 1 => Some((x, y)),
                    // Some(s) if s.get() == cur => Some((x, y)),
                    // Some(s) if s.get() < cur => Some((x, y)),
                    // Some(s) if s.get() == cur - 1 || s.get() == cur || s.get() > cur => {
                    Some(s) if s.get() >= cur - 1 => {
                        let ch = char::from(s.get());
                        println!("> {ch} {:08b}", s.get());
                        Some((x, y))
                    }

                    Some(s) => {
                        let ch = char::from(s.get());
                        println!("! {ch} {:08b}", s.get());
                        None
                    }
                    None => None,
                }
            })
            .for_each(|(x, y)| {
                println!("({x},{y})");
                map[y][x].set(map[y][x].get() | 1 << 7);
                q.push_back(((x, y), len + 1));
            });

        println!("{q:?}");
        l += 1;
    }

    None
}

fn walk(row: isize, col: isize, cur: u8, len: u32, map: &[&[Cell<u8>]]) -> Option<u32> {
    // let ch = char::from(cur);
    // println!("row={row} col={col} cur={cur} char='{ch}' len={len}");
    if cur == b'E' {
        println!("Done (len={len})");
        return Some(len);
    }
    if seen(&Cell::new(cur)) {
        // println!("SEEN");
        return None;
    }
    NEIB.iter()
        .filter_map(|(dx, dy)| {
            let y = row + dy;
            let x = col + dx;
            match map.get(y as usize).and_then(|row| row.get(x as usize)) {
                Some(s) if seen(s) => {
                    // let ch = char::from(flip(s.get()));
                    // println!("+ {ch} neib seen ({x},{y})");
                    None
                }
                Some(s) if cur == b'z' && s.get() == b'E' => {
                    walk(y, x, b'E', len + 1, map)
                    // let len = len + 1;
                    // println!("END! ({x},{y}) len={len}");
                    // Some(len)
                }
                Some(s) if s.get() == b'E' => {
                    println!("E not yet");
                    None
                }
                Some(s) if s.get() == cur + 1 => {
                    with_flip(s, x, y, || walk(y, x, cur + 1, len + 1, map))
                    // flip(s);
                    // walk(y as isize, x as isize, cur + 1, len + 1, map)
                    // flip(s);
                }
                Some(s) if s.get() == cur => {
                    // mark(s)
                    with_flip(s, x, y, || walk(y, x, cur, len + 1, map))
                }
                Some(s) if s.get() < cur => {
                    // mark(s)
                    with_flip(s, x, y, || walk(y, x, s.get(), len + 1, map))
                }
                // _ => walk(row, col, 0, len + 1, map),
                None => {
                    // println!("no neib ({x},{y})");
                    None
                }
                Some(s) => {
                    // let ch = char::from(s.get());
                    // println!("! {ch} ({x},{y})");
                    None
                }
            }
        })
        .inspect(|v| {
            dbg!(v);
        })
        .min()

    // i += 1
    // walk(_, _)
    // i -= 1
    // res = res.min(cur)
    // next:
    // - Some()
    // - prev+1
    // - or not vis and = cur
    // - not vis and < cur
}

fn seen(square: &Cell<u8>) -> bool {
    let seen = square.get() & 1 << 7 != 0;
    // println!("{} is seen {seen}", char::from(square.get()));
    seen
}

fn flip(v: u8) -> u8 {
    if v & 1 << 7 != 0 {
        v & !(1 << 7)
    } else {
        v | 1 << 7
    }
}

fn with_flip<F>(square: &Cell<u8>, x: isize, y: isize, f: F) -> Option<u32>
where
    F: FnOnce() -> Option<u32>,
{
    // println!("> {} ({x},{y})", char::from(square.get()));
    // dbg!(square);
    square.set(square.get() | 1 << 7);
    // dbg!(square);
    let res = f();
    // println!("{res:?}");
    // dbg!(square);
    square.set(square.get() & !(1 << 7));
    // println!("< {} ({x},{y})", char::from(square.get()));
    // dbg!(square);
    res
}
