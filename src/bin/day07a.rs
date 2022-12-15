const LIMIT: u32 = 100_000;

pub fn main() {
    let cmds = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    let cmds = include_str!("day07.input");

    let mut res = vec![];
    let mut stak = vec![];
    let mut lines = cmds.lines().skip(1);
    let mut cwd: Vec<(&str, Option<u32>)> = vec![];
    let mut pop_all = false;

    loop {
        let line = lines.next().or_else(|| {
            if stak.len() > 0 && pop_all {
                Some("$ cd ..")
            } else {
                None
            }
        });

        match line.and_then(|l| l.rsplit_once(' ')) {
            Some(("$", "ls")) => continue,
            Some(("$ cd", "..")) => {
                let sum = cwd.iter().map(|e| e.1.unwrap()).sum::<u32>();

                if sum < LIMIT {
                    res.push(sum);
                }

                let (mut parent, this_dir): (Vec<(&str, Option<u32>)>, &str) = stak.pop().unwrap();

                parent
                    .iter_mut()
                    .find(|v| v.0 == this_dir)
                    .map(|v| v.1.replace(sum));

                cwd = parent;
            }
            Some(("$ cd", dir)) => {
                stak.push((cwd, dir));
                cwd = vec![];
            }
            Some(("dir", dir)) => cwd.push((dir, None)),
            Some((size, file)) => cwd.push((file, Some(size.parse::<u32>().unwrap()))),
            None if !pop_all => pop_all = true,
            None => break,
        }
    }

    println!("{}", res.into_iter().sum::<u32>());
}
