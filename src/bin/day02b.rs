const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSE: u32 = 0;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

pub fn main() {
    // let answ = "\
    // A Y
    // B X
    // C Z"
    let answ = include_str!("day02.input")
        .lines()
        .map(|s| {
            s.split_once(' ')
                .map(|(em, res)| {
                    (
                        match em {
                            "A" => ROCK,
                            "B" => PAPER,
                            "C" => SCISSORS,
                            _ => unreachable!(),
                        },
                        match res {
                            "X" => LOSE,
                            "Y" => DRAW,
                            "Z" => WIN,
                            _ => unreachable!(),
                        },
                    )
                })
                .unwrap()
        })
        .fold(0, |score, (em, res)| {
            score
                + res
                + match (em, res) {
                    (ROCK, LOSE) => SCISSORS,
                    (ROCK, WIN) => PAPER,
                    (PAPER, LOSE) => ROCK,
                    (PAPER, WIN) => SCISSORS,
                    (SCISSORS, LOSE) => PAPER,
                    (SCISSORS, WIN) => ROCK,
                    (_, DRAW) => em,
                    _ => unreachable!(),
                }
        });

    println!("{}", answ);
}
