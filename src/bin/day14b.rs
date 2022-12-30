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
    let mut sum = 1;
    let mut x = s.0;
    let mut y = s.1;

    loop {
        y += 1;

        if !map[y][x] {
            continue;
        }

        if !map[y][x - 1] {
            x -= 1;
            continue;
        }

        if !map[y][x + 1] {
            x += 1;
            continue;
        }

        if x == s.0 && y == s.1 + 1 {
            break;
        }

        map[y - 1][x] = true;
        sum += 1;
        x = s.0;
        y = s.1;
    }

    println!("{}", sum);
}
