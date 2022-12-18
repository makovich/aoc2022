pub fn main() {
    let inpt = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    let inpt = include_str!("day11.input");

    let mut mnks = inpt.split("\n\n").fold(vec![], |mut mnks, line| {
        let m = line
            .lines()
            .skip(1)
            .flat_map(|l| l.split_once(':'))
            .map(|(_, l)| l)
            .map(|l| l.trim())
            .collect::<Vec<_>>();

        let last = |line: &str| line.split(' ').last().map(|w| w.parse().ok()).unwrap();

        mnks.push(Mnky {
            q: m[0].split(", ").filter_map(|w| w.parse().ok()).collect(),
            o: m[1]
                .split_once("= old ")
                .map(|(_, func)| (func.chars().nth(0).unwrap(), last(func)))
                .unwrap(),
            t: last(m[2]).unwrap(),
            y: last(m[3]).unwrap(),
            n: last(m[4]).unwrap(),
            i: 0,
        });

        mnks
    });

    let gcd = mnks.iter().map(|m| m.t).product::<usize>();

    for _round in 0..10000 {
        for i in 0..mnks.len() {
            mnks[i]
                .worry(gcd)
                .into_iter()
                .for_each(|(i, item)| mnks[i].q.push(item));
        }
    }

    mnks.sort_unstable_by(|a, b| b.i.cmp(&a.i));

    let answ = mnks.iter().map(|m| m.i).take(2).product::<usize>();

    println!("{:?}", answ);
}

#[derive(Debug)]
struct Mnky {
    q: Vec<usize>,
    o: (char, Option<usize>),
    t: usize,
    y: usize,
    n: usize,
    i: usize,
}

impl Mnky {
    fn worry(&mut self, gcd: usize) -> Vec<(usize, usize)> {
        let mut r = vec![];

        loop {
            if self.q.is_empty() {
                break;
            }

            let item = self.q.remove(0);
            let item = match self.o.0 {
                '*' => item * self.o.1.unwrap_or(item),
                '+' => item + self.o.1.unwrap_or(item),
                _ => unreachable!(),
            };

            let item = item % gcd;
            let next = if item % self.t == 0 { self.y } else { self.n };

            self.i += 1;

            r.push((next, item));
        }

        r
    }
}
