fn rotate(code: &str, current: i32) -> i32 {
    let sign = code.starts_with('R');
    let amt = code[1..]
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("Couldn't parse {} as number", code));

    let sign = if sign { 1 } else { -1 };
    let next = (current + amt * sign) % 100;

    let ret = if next < 0 { next + 100 } else { next };

    // println!("Sign is {}, amt is {}, ret is {}", sign, amt, ret);

    ret
}

pub fn do_day1_p1(input: &str) -> i32 {
    let mut count = 0;
    let mut current = 50;
    for word in input.lines() {
        current = rotate(word, current);
        if current == 0 {
            count += 1;
        }
    }

    count
}

pub fn day1_p1() {
    let words = include_str!("input/day1_input.txt");

    let count = do_day1_p1(words);
    println!("Count is {}", count);
}

#[test]
fn test_day1p1() {
    let words = include_str!("input/day1_example.txt");

    let count = do_day1_p1(words);
    assert_eq!(3, count);
}
