fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect::<Vec<_>>()
}

fn do_day9p1(input: &str) -> u64 {
    let mut max_area = 0;

    let coords = parse_input(input);

    for i in 0..coords.len() {
        for j in 0..coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];

            let width = x2.abs_diff(x1) + 1;
            let height = y2.abs_diff(y1) + 1;

            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

pub fn p1() {
    let input = include_str!("input/day9_input.txt");

    let ret = do_day9p1(input);

    println!("Day 9 Part 1: The largest area is {}", ret);
}

#[test]
fn test_day9p1() {
    let input = include_str!("input/day9_example.txt");

    let ret = do_day9p1(input);

    assert_eq!(ret, 50);
}
