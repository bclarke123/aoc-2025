fn parse_board(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn remove_rolls(
    board: &mut [Vec<u8>],
    cache: &mut Vec<(usize, usize)>,
    max_around: usize,
) -> usize {
    cache.clear();

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            let mut neighbours = 0;

            let cell = board[y][x];
            if cell == 0 {
                continue;
            }

            let start_x = x.saturating_sub(1);
            let start_y = y.saturating_sub(1);

            let end_x = (x + 1).min(board[y].len() - 1);
            let end_y = (y + 1).min(board.len() - 1);

            'test: for iy in start_y..=end_y {
                for ix in start_x..=end_x {
                    if iy == y && ix == x {
                        continue;
                    }

                    if board[iy][ix] > 0 {
                        neighbours += 1;
                    }

                    if neighbours >= max_around {
                        break 'test;
                    }
                }
            }

            if neighbours < max_around {
                cache.push((y, x));
            }
        }
    }

    let ret = cache.len();

    for (y, x) in cache {
        board[*y][*x] = 0;
    }

    ret
}

fn do_day4p1(input: &str, max_around: usize) -> usize {
    let mut board = parse_board(input);

    remove_rolls(&mut board, &mut vec![], max_around)
}

fn do_day4p2(input: &str, max_around: usize) -> usize {
    let mut ret = 0;
    let mut board = parse_board(input);
    let mut cache = vec![];

    loop {
        let last_removed = remove_rolls(&mut board, &mut cache, max_around);
        ret += last_removed;

        if last_removed == 0 {
            break;
        }
    }

    ret
}

pub fn p1() {
    let input = include_str!("input/day4_input.txt");

    let ret = do_day4p1(input, 4);
    println!("Day 4 Part 1: The number of accessible rolls is {}", ret);
}

pub fn p2() {
    let input = include_str!("input/day4_input.txt");

    let ret = do_day4p2(input, 4);
    println!("Day 4 Part 2: The number of accessible rolls is {}", ret);
}

#[test]
fn test_day4p1() {
    let input = include_str!("input/day4_example.txt");

    let ret = do_day4p1(input, 4);
    assert_eq!(ret, 13);
}

#[test]
fn test_day4p2() {
    let input = include_str!("input/day4_example.txt");

    let ret = do_day4p2(input, 4);
    assert_eq!(ret, 43);
}
