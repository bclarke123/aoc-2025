fn get_hilo(input: &str) -> Vec<Vec<u64>> {
    input
        .split(",")
        .map(|p| {
            p.split("-")
                .map(|s| {
                    s.trim().parse::<u64>()
                        .unwrap_or_else(|_| panic!("Couldn't parse {} as a number", s))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_invalid(input: &str, len: usize) -> bool {
    let strlen = input.len();
    if strlen % len != 0 {
        return false;
    }

    let first = &input[0..len];
    for i in 1..strlen / len {
        if &input[i..i+len] != first {
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

        for i in lo ..= hi {
            let str = i.to_string();
            let len = str.len() / 2;
            if str[0..len] == str[len..] {
                ret += i;
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

#[test]
fn test_day2p1() {
    let input = include_str!("input/day2_example.txt");

    let ret = do_day2p1(input);

    assert_eq!(ret, 1227775554);
}
