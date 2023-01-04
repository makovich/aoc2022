const W: usize = 1000;
const H: usize = 200;

pub fn main() {
    let pths = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    let pths = include_str!("day14.input");

    let mut bot = 0;

    let mut map = pths.lines().fold([[false; W]; H], |mut m, line| {
        line.split(" -> ")
            .filter_map(|xy| xy.split_once(','))
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .reduce(|(x1, y1), (x2, y2)| {
                bot = bot.max(y1.max(y2));

                for x in x1.min(x2)..=x1.max(x2) {
                    for y in y1.min(y2)..=y1.max(y2) {
                        m[y][x] = true;
                    }
                }

                (x2, y2)
            });
        m
    });

    for x in 0..W {
        map[bot + 2][x] = true;
    }

    let s = (500, 0);
    let h = bot + 2;
    let mut rks = 0;

    for y in 1..h {
        rks += map[y][s.0 - y] as usize;
        rks += map[y][s.0 + y] as usize;

        for x in (s.0 - y + 1)..(s.0 + y) {
            rks += map[y][x] as usize;

            if map[y][x - 1] && map[y][x] && map[y][x + 1] {
                map[y + 1][x] = true;
            }
        }
    }

    let sum = h * h - rks;

    println!("{}", sum);
}
