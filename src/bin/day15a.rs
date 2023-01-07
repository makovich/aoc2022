use std::collections::{BinaryHeap, HashSet};

pub fn main() {
    let (target, input) = (
        10i32,
        "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    );

    let (target, input) = (2000000i32, include_str!("day15.input"));

    let mut beacons = HashSet::new();

    let range = input
        .lines()
        .filter_map(|l| l.split_once(':'))
        .filter_map(|(a, b)| a.split_once(',').zip(b.split_once(',')))
        .map(|((x1, y1), (x2, y2))| {
            let parse = |s: &str| {
                s.split_once('=')
                    .map(|(_, v)| v.parse::<i32>().unwrap())
                    .unwrap()
            };

            (parse(x1), parse(y1), parse(x2), parse(y2))
        })
        .filter_map(|(x1, y1, x2, y2)| {
            (y2 == target).then(|| beacons.insert(x2));

            let md = x1.abs_diff(x2) + y1.abs_diff(y2);
            let td = target.abs_diff(y1);

            md.checked_sub(td).map(|w| (x1 - w as i32, x1 + w as i32))
        })
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec()
        .into_iter()
        .reduce(|(a, _), (_, d)| (a, d))
        .expect("single range by puzzle's description");

    println!("{:?}", range.0.abs_diff(range.1));
}
