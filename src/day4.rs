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

fn remove_rolls(board: Vec<Vec<usize>>, max_around: usize) -> (Vec<Vec<usize>>, usize) {
    let mut ret = 0;
    let mut new_board = board.clone();

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

                    neighbours += cell;

                    if neighbours >= max_around {
                        break 'test;
                    }
                }
            }

            if neighbours < max_around {
                ret += 1;
                new_board[y][x] = 0;
            }
        }
    }

    (new_board, ret)
}

fn do_day4p1(input: &str, max_around: usize) -> usize {
    let board = parse_board(input);

    remove_rolls(board, max_around).1
}

fn do_day4p2(input: &str, max_around: usize) -> usize {
    let mut ret = 0;
    let mut last_removed;
    let mut board = parse_board(input);

    loop {
        (board, last_removed) = remove_rolls(board, max_around);
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
    println!("The number of accessible rolls is {}", ret);
}

pub fn p2() {
    let input = include_str!("input/day4_input.txt");

    let ret = do_day4p2(input, 4);
    println!("The number of accessible rolls is {}", ret);
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
