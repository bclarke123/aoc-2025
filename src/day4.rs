fn parse_board(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn remove_rolls(board: &mut [Vec<usize>], max_around: usize) -> usize {
    let mut ret = 0;

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            let mut neighbours = 0;

            let cell = board[y][x];
            if cell == 0 {
                continue;
            }

            let start_x = 1.max(x) - 1;
            let start_y = 1.max(y) - 1;

            let end_x = (board[y].len() - 2).min(x) + 1;
            let end_y = (board.len() - 2).min(y) + 1;

            'test: for (iy, row) in board.iter().enumerate().take(end_y + 1).skip(start_y) {
                for (ix, cell) in row.iter().enumerate().take(end_x + 1).skip(start_x) {
                    if iy == y && ix == x {
                        continue;
                    }

                    if *cell > 0 {
                        neighbours += 1;
                    }

                    if neighbours >= max_around {
                        break 'test;
                    }
                }
            }

            if neighbours < max_around {
                ret += 1;
                board[y][x] = 2;
            }
        }
    }

    for row in board {
        for cell in row {
            if *cell == 2 {
                *cell = 0;
            }
        }
    }

    ret
}

fn do_day4p1(input: &str, max_around: usize) -> usize {
    let mut board = parse_board(input);

    remove_rolls(&mut board, max_around)
}

fn do_day4p2(input: &str, max_around: usize) -> usize {
    let mut ret = 0;
    let mut board = parse_board(input);

    loop {
        let last_removed = remove_rolls(&mut board, max_around);
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
