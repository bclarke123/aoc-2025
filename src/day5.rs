struct Food {
    fresh: Vec<(usize, usize)>,
    available: Vec<usize>,
}

impl Food {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();

        let fresh = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (lo, hi) = line.split_once('-').unwrap();
                (lo.parse().unwrap(), hi.parse().unwrap())
            })
            .collect::<Vec<(usize, usize)>>();

        let available = lines.map(|x| x.parse().unwrap()).collect::<Vec<usize>>();

        Self { fresh, available }
    }

    fn enumerate(&self) -> Vec<(usize, bool)> {
        self.available
            .iter()
            .map(|&n| (n, self.fresh.iter().any(|(lo, hi)| n >= *lo && n <= *hi)))
            .collect::<Vec<_>>()
    }

    fn total_fresh(&mut self) -> u64 {
        self.fresh.sort_by_key(|&(lo, _)| lo);

        let mut combined = Vec::<(usize, usize)>::new();

        for &cur in &self.fresh {
            let lo = cur.0;
            let hi = cur.1;

            let prev = combined.pop();

            if prev.is_none() {
                combined.push((lo, hi));
                continue;
            }

            let prev = prev.unwrap();

            if prev.1 < lo {
                combined.push(prev);
                combined.push((lo, hi));
                continue;
            }

            combined.push((prev.0, hi.max(prev.1)));
        }

        combined.iter().map(|(lo, hi)| (hi - lo + 1) as u64).sum()
    }
}

fn do_part1(input: &str) -> usize {
    let food = Food::new(input);
    food.enumerate()
        .iter()
        .filter(|(_, is_fresh)| *is_fresh)
        .count()
}

fn do_part2(input: &str) -> u64 {
    Food::new(input).total_fresh()
}

pub fn p1() {
    let input = include_str!("input/day5_input.txt");
    let ret = do_part1(input);

    println!("Day 5 Part 1: {} ingredients are fresh", ret);
}

pub fn p2() {
    let input = include_str!("input/day5_input.txt");
    let ret = do_part2(input);

    println!("Day 5 Part 2: {} ingredients are fresh", ret);
}

#[test]
fn test_day5p1() {
    let input = include_str!("input/day5_example.txt");
    let ret = do_part1(input);

    assert_eq!(3, ret);
}

#[test]
fn test_day5p2() {
    let input = include_str!("input/day5_example.txt");
    let ret = do_part2(input);

    assert_eq!(14, ret);
}
