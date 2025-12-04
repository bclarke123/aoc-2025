fn get_hilo(input: &str) -> Vec<Vec<u64>> {
    input
        .split(",")
        .map(|p| {
            p.split("-")
                .map(|s| {
                    s.trim()
                        .parse::<u64>()
                        .unwrap_or_else(|_| panic!("Couldn't parse {} as a number", s))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_invalid(input: &str, len: usize) -> bool {
    let strlen = input.len();
    if !strlen.is_multiple_of(len) {
        return false;
    }

    let first = &input[0..len];
    for i in 1..strlen / len {
        let start = len * i;
        let end = start + len;
        if &input[start..end] != first {
            return false;
        }
    }

    true
}

fn do_day2p1(input: &str) -> u64 {
    let hilo = get_hilo(input);
    let mut ret = 0;

    for item in hilo {
        let lo = *item.first().unwrap();
        let hi = *item.last().unwrap();

        for i in lo..=hi {
            let str = i.to_string();
            let len = str.len();

            if len % 2 == 0 && is_invalid(&str, len / 2) {
                ret += i;
            }
        }
    }

    ret
}

fn do_day2p2(input: &str) -> u64 {
    let hilo = get_hilo(input);
    let mut ret = 0;

    for item in hilo {
        let lo = *item.first().unwrap();
        let hi = *item.last().unwrap();

        for i in lo..=hi {
            let str = i.to_string();
            let len = str.len() / 2;

            for n in 0..len {
                if is_invalid(&str, len - n) {
                    ret += i;
                    break;
                }
            }
        }
    }

    ret
}

pub fn day2_p1() {
    let input = include_str!("input/day2_input.txt");

    let ret = do_day2p1(input);
    println!("The invalid codes add up to {}", ret);
}

pub fn day2_p2() {
    let input = include_str!("input/day2_input.txt");

    let ret = do_day2p2(input);
    println!("The invalid codes add up to {}", ret);
}

#[test]
fn test_day2p1() {
    let input = include_str!("input/day2_example.txt");

    let ret = do_day2p1(input);

    assert_eq!(ret, 1227775554);
}

#[test]
fn test_day2p2() {
    let input = include_str!("input/day2_example.txt");

    let ret = do_day2p2(input);

    assert_eq!(ret, 4174379265);
}
