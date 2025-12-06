struct Board {
    numbers: Vec<u64>,
    operators: Vec<char>,
    width: usize,
}

impl Board {
    fn new(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();

        let numbers_2d = lines[..lines.len() - 1]
            .iter()
            .map(|l| {
                l.trim()
                    .split_whitespace()
                    .map(|w| w.trim().parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let operators = lines
            .last()
            .unwrap()
            .split_whitespace()
            .map(|w| w.trim().chars().next().unwrap())
            .collect::<Vec<_>>();

        let width = numbers_2d[0].len();
        let numbers = numbers_2d.into_iter().flatten().collect::<Vec<_>>();

        Self {
            numbers,
            operators,
            width,
        }
    }

    fn num(&self, x: usize, y: usize) -> u64 {
        self.numbers[y * self.width + x]
    }

    fn op(&self, x: usize, lh: u64, rh: u64) -> u64 {
        match self.operators[x] {
            '+' => u64::saturating_add(lh, rh),
            '*' => u64::saturating_mul(lh, rh),
            _ => panic!("Unknown operator {}", self.operators[x]),
        }
    }

    fn numbers(&self, x: usize) -> Vec<u64> {
        self.numbers
            .iter()
            .skip(x)
            .step_by(self.width)
            .copied()
            .collect()
    }
}

fn do_day6p1(input: &str) -> u64 {
    let board = Board::new(input);
    (0..board.width)
        .map(|x| {
            board
                .numbers(x)
                .iter()
                .skip(1)
                .fold(board.num(x, 0), |acc, &n| board.op(x, acc, n))
        })
        .sum()
}

pub fn p1() {
    let input = include_str!("input/day6_input.txt");

    let ret = do_day6p1(input);

    println!("Day 6 Part 1: The grand total is {}", ret);
}

#[test]
fn test_day6p1() {
    let input = include_str!("input/day6_example.txt");

    let ret = do_day6p1(input);

    assert_eq!(ret, 4277556);
}
