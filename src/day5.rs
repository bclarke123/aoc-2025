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

    fn fresh_available(&self) -> usize {
        self.available
            .iter()
            .filter(|n| self.fresh.iter().any(|&(lo, hi)| (lo..=hi).contains(n)))
            .count()
    }

    fn total_fresh(&mut self) -> u64 {
        self.fresh.sort_by_key(|&(lo, _)| lo);

        let mut combined = Vec::<(usize, usize)>::new();

        for &(lo, hi) in &self.fresh {
            if combined.is_empty() {
                combined.push((lo, hi));
                continue;
            }

            let (plo, phi) = combined.pop().unwrap();

            if phi < lo {
                combined.push((plo, phi));
                combined.push((lo, hi));
                continue;
            }

            combined.push((plo, hi.max(phi)));
        }

        combined.iter().map(|(lo, hi)| (hi - lo + 1) as u64).sum()
    }
}

fn do_part1(input: &str) -> usize {
    Food::new(input).fresh_available()
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
