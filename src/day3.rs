fn get_joltages(bank: &str) -> Vec<u64> {
    bank.chars()
        .map(|c| {
            c.to_digit(10)
                .unwrap_or_else(|| panic!("Invalid character: {}", c))
                .into()
        })
        .collect::<Vec<_>>()
}

fn max_joltages(bank: Vec<u64>, _digits: usize) -> u64 {
    let mut hi_val = 0;

    for a in 0..bank.len() {
        for b in a + 1..bank.len() {
            let val = bank[a] * 10 + bank[b];
            hi_val = hi_val.max(val)
        }
    }

    hi_val
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

pub fn day3_p1() {
    let input = include_str!("input/day3_input.txt");

    let ret = do_day3p1(input);
    println!("The sum of maximum joltages (2) is {}", ret);
}

pub fn day3_p2() {
    let input = include_str!("input/day3_input.txt");

    let ret = do_day3p2(input);
    println!("The sum of maximum joltages (12) is {}", ret);
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
