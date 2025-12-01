fn rotate(code: &str) -> (i32, i32) {
    let sign = code.starts_with('R');
    let amt = code[1..]
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("Couldn't parse {} as number", code));

    let mul = if sign { 1 } else { -1 };

    (amt, mul)
}

pub fn do_day1_p1(input: &str) -> i32 {
    let mut count = 0;
    let mut current = 50;

    for word in input.lines() {
        let (amt, sign) = rotate(word);
        let next = (current + amt * sign) % 100;
        let ret = if next < 0 { next + 100 } else { next };

        current = ret;

        if current == 0 {
            count += 1;
        }
    }

    count
}

pub fn do_day1_p2(input: &str) -> i32 {
    let mut count = 0;
    let mut current = 50;

    for word in input.lines() {
        let (amt, sign) = rotate(word);
        let next = (current + amt * sign) % 100;
        let next = if next < 0 { next + 100 } else { next };
        let circles = amt / 100;
        let fract = amt % 100;

        count += circles;

        if (sign > 0 && current != 0 && next < current)
            || (sign < 0 && current != 0 && next > current)
            || (fract != 0 && next == 0)
        {
            count += 1;
        }

        current = next;
    }

    count
}

pub fn day1_p1() {
    let words = include_str!("input/day1_input.txt");

    let count = do_day1_p1(words);
    println!("Part 1: Count is {}", count);
}

pub fn day1_p2() {
    let words = include_str!("input/day1_input.txt");

    let count = do_day1_p2(words);
    println!("Part 2: Count is {}", count);
}

#[test]
fn test_day1p1() {
    let words = include_str!("input/day1_example.txt");

    let count = do_day1_p1(words);
    assert_eq!(3, count);
}

#[test]
fn test_day1p2() {
    let words = include_str!("input/day1_example.txt");

    let count = do_day1_p2(words);
    assert_eq!(6, count);

    let ten = do_day1_p2("L1000");
    assert_eq!(10, ten);

    let ten = do_day1_p2("R1000");
    assert_eq!(10, ten);

    let three = do_day1_p2("R50\nL100\nR100");
    assert_eq!(3, three);
}
