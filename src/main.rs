macro_rules! aoc2022_main {
    ( $( $day:tt ),+ $(,)? ) => {
        mod bin {
            $( pub mod $day; )+
        }

        fn main() {
            let mut tx = Vec::new();

        $(
            let t = std::time::Instant::now();
            println!(">> {}", stringify!($day));
            bin::$day::main();
            tx.push((stringify!($day), t.elapsed()));
        )+

            println!("\n\nTotals:");
            let mut sum = std::time::Duration::new(0,0);
            for (d, t) in tx {
                println!("{:>10}   {:?}", d, t);
                sum += t;
            }
            println!("            ------------");
            println!("             {:?}", sum);
        }
    };
}

aoc2022_main! {
    day01a,
}
