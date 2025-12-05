fn get_joltages(bank: &str) -> Vec<u64> {
    bank.chars()
        .map(|c| {
            c.to_digit(10)
                .unwrap_or_else(|| panic!("Invalid character: {}", c))
                .into()
        })
        .collect::<Vec<_>>()
}

fn max_joltages(bank: Vec<u64>, digits: usize) -> u64 {
    let mut stack: Vec<u64> = vec![];

    for (i, &digit) in bank.iter().enumerate() {
        while let Some(&top) = stack.last() {
            let current_stack = stack.len();
            let remaining = bank.len() - i;
            let min_remaining = current_stack + remaining - 1;

            if digit > top && digits <= min_remaining {
                stack.pop();
            } else {
                break;
            }
        }

        if stack.len() < digits {
            stack.push(digit);
        }
    }

    stack.iter().fold(0, |acc, x| acc * 10 + x)
}

fn do_day3p1(input: &str) -> u64 {
    input
        .lines()
        .map(get_joltages)
        .map(|bank| max_joltages(bank, 2))
        .sum()
}

fn do_day3p2(input: &str) -> u64 {
    input
        .lines()
        .map(get_joltages)
        .map(|bank| max_joltages(bank, 12))
        .sum()
}

pub fn p1() {
    let input = include_str!("input/day3_input.txt");

    let ret = do_day3p1(input);
    println!("Day 3 Part 1: The sum of maximum joltages (2) is {}", ret);
}

pub fn p2() {
    let input = include_str!("input/day3_input.txt");

    let ret = do_day3p2(input);
    println!("Day 3 Part 2: The sum of maximum joltages (12) is {}", ret);
}

#[test]
fn test_day3p1() {
    let input = include_str!("input/day3_example.txt");

    let ret = do_day3p1(input);

    assert_eq!(ret, 357);
}

#[test]
fn test_day3p2() {
    let input = include_str!("input/day3_example.txt");

    let ret = do_day3p2(input);

    assert_eq!(ret, 3121910778619);
}
