fn op(operator: char, lh: u64, rh: u64) -> u64 {
    match operator {
        '+' => lh.saturating_add(rh),
        '*' => lh.saturating_mul(rh),
        _ => panic!("Unknown operator {}", operator),
    }
}

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
                l.split_whitespace()
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

    fn numbers(&self, x: usize) -> Vec<u64> {
        self.numbers
            .iter()
            .skip(x)
            .step_by(self.width)
            .copied()
            .collect()
    }

    fn op(&self, x: usize, lh: u64, rh: u64) -> u64 {
        op(self.operators[x], lh, rh)
    }

    fn solve(&self) -> u64 {
        (0..self.width)
            .map(|x| {
                self.numbers(x)
                    .iter()
                    .skip(1)
                    .fold(self.num(x, 0), |acc, &n| self.op(x, acc, n))
            })
            .sum()
    }
}

struct Board2 {
    numbers: Vec<Vec<u64>>,
    operators: Vec<char>,
}

impl Board2 {
    fn new(input: &str) -> Self {
        let chars = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let operators = chars.last().unwrap();
        let width = chars.iter().map(|n| n.len()).max().unwrap();

        let mut numbers: Vec<Vec<u64>> = vec![];
        let mut buf: Vec<String> = vec![];

        for i in 0..width {
            let c = operators.get(i).unwrap_or(&' ');

            if *c != ' ' && i > 0 {
                let x = buf
                    .iter()
                    .filter_map(|s| s.trim().parse::<u64>().ok())
                    .collect::<Vec<_>>();

                numbers.push(x);
                buf.clear();
            }

            let next_str = chars[..chars.len() - 1]
                .iter()
                .map(|line| line.get(i).unwrap_or(&' '))
                .collect::<String>();
            buf.push(next_str);
        }

        let x = buf
            .iter()
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .collect::<Vec<_>>();
        numbers.push(x);

        let operators = operators
            .iter()
            .copied()
            .filter(|x| *x != ' ')
            .collect::<Vec<_>>();

        Self { numbers, operators }
    }

    fn solve(&self) -> u64 {
        let mut ret = 0;

        for (i, &o) in self.operators.iter().enumerate() {
            let arr = &self.numbers[i];
            let next = arr.iter().skip(1).fold(arr[0], |a, c| op(o, a, *c));
            ret += next;
        }

        ret
    }
}

fn do_day6p1(input: &str) -> u64 {
    Board::new(input).solve()
}

fn do_day6p2(input: &str) -> u64 {
    Board2::new(input).solve()
}

pub fn p1() {
    let input = include_str!("input/day6_input.txt");

    let ret = do_day6p1(input);

    println!("Day 6 Part 1: The grand total is {}", ret);
}

pub fn p2() {
    let input = include_str!("input/day6_input.txt");

    let ret = do_day6p2(input);

    println!("Day 6 Part 2: The grand total is {}", ret);
}

#[test]
fn test_day6p1() {
    let input = include_str!("input/day6_example.txt");

    let ret = do_day6p1(input);

    assert_eq!(ret, 4277556);
}

#[test]
fn test_day6p2() {
    let input = include_str!("input/day6_example.txt");

    let ret = do_day6p2(input);

    assert_eq!(ret, 3263827);
}
