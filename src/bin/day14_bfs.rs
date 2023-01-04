const W: usize = 1000;
const H: usize = 200;

const FREE: u8 = 0;
const ROCK: u8 = 2;
const NEXT: u8 = 1;

pub fn main() {
    let pths = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    let pths = include_str!("day14.input");

    let mut bot = 0;

    let mut map = pths.lines().fold([[0u8; W]; H], |mut m, line| {
        line.split(" -> ")
            .filter_map(|xy| xy.split_once(','))
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .reduce(|(x1, y1), (x2, y2)| {
                bot = bot.max(y1.max(y2));

                for x in x1.min(x2)..=x1.max(x2) {
                    for y in y1.min(y2)..=y1.max(y2) {
                        m[y][x] = ROCK;
                    }
                }

                (x2, y2)
            });
        m
    });

    for x in 0..W {
        map[bot + 2][x] = ROCK;
    }

    let s = (500, 0);
    let mut sum = 1;

    map[s.1][s.0] = NEXT;

    for y in 0..=bot {
        for x in (s.0 - y)..=(s.0 + y) {
            if map[y][x] != NEXT {
                continue;
            }

            for x in [x - 1, x, x + 1] {
                if map[y + 1][x] != FREE {
                    continue;
                }

                map[y + 1][x] = NEXT;
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}
