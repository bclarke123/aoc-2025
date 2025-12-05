struct Food {
    fresh: Vec<(usize, usize)>,
    available: Vec<usize>,
}

impl Food {
    fn new(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();

        let fresh = lines.iter().take_while(|line| !line.is_empty())
            .map(|line| line.split_once('-').unwrap())
            .map(|(lo, hi)| (lo.parse().unwrap(), hi.parse().unwrap()))
            .collect::<Vec<(usize, usize)>>();

        let available = lines.iter().skip(fresh.len() + 1).map(|x| x.parse().unwrap()).collect::<Vec<usize>>();

        Self {
            fresh,
            available
        }
    }

    fn enumerate(&self) -> Vec<(usize, bool)> {
        self.available.iter().map(|&n| {
            (n, self.fresh.iter().any(|(lo, hi)| n >= *lo && n <= *hi))
        }).collect::<Vec<_>>()
    }
}

fn do_part1(input: &str) -> usize {
    let food = Food::new(input);
    food.enumerate().iter().filter(|(_, is_fresh)| *is_fresh).count()
}

pub fn p1() {
    let input = include_str!("input/day5_input.txt");
    let ret = do_part1(input);

    println!("Day 5 Part 1: {} ingredients are fresh", ret);
}

#[test]
fn test_day5p1() {
    let input = include_str!("input/day5_example.txt");
    let ret = do_part1(input);

    assert_eq!(3, ret);
}
