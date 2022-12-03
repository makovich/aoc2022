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
    // C Z
    // "
    let answ = include_str!("day02.input")
        .lines()
        .map(|s| {
            s.split_once(' ')
                .map(|(em, us)| {
                    (
                        match em {
                            "A" => ROCK,
                            "B" => PAPER,
                            "C" => SCISSORS,
                            _ => unreachable!(),
                        },
                        match us {
                            "X" => ROCK,
                            "Y" => PAPER,
                            "Z" => SCISSORS,
                            _ => unreachable!(),
                        },
                    )
                })
                .unwrap()
        })
        .fold(0, |score, (em, us)| {
            score
                + us
                + match (em, us) {
                    (ROCK, ROCK) | (PAPER, PAPER) | (SCISSORS, SCISSORS) => DRAW,
                    (ROCK, PAPER) | (PAPER, SCISSORS) | (SCISSORS, ROCK) => WIN,
                    _ => LOSE,
                }
        });

    println!("{}", answ);
}
