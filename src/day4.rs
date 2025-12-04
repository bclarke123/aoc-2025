fn parse_board(input: &str) -> (Vec<u8>, usize, usize) {
    let input_2d = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    assert!(!input_2d.is_empty());

    let height = input_2d.len();
    let width = input_2d[0].len();

    (
        input_2d.iter().flatten().copied().collect::<Vec<_>>(),
        width,
        height,
    )
}

struct Warehouse {
    board: Vec<u8>,
    width: usize,
    height: usize,
    cache: Vec<usize>,
    max_around: usize,
}

impl Warehouse {
    fn new(input: &str, max_around: usize) -> Self {
        let board = parse_board(input);

        Self {
            board: board.0,
            width: board.1,
            height: board.2,
            cache: vec![],
            max_around,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn remove_rolls(&mut self) -> usize {
        self.cache.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut neighbours = 0;
                let index = self.index(x, y);

                let cell = self.board[index];
                if cell == 0 {
                    continue;
                }

                let start_x = x.saturating_sub(1);
                let start_y = y.saturating_sub(1);

                let end_x = (x + 1).min(self.width - 1);
                let end_y = (y + 1).min(self.height - 1);

                'test: for iy in start_y..=end_y {
                    for ix in start_x..=end_x {
                        if iy == y && ix == x {
                            continue;
                        }

                        if self.board[self.index(ix, iy)] > 0 {
                            neighbours += 1;
                        }

                        if neighbours >= self.max_around {
                            break 'test;
                        }
                    }
                }

                if neighbours < self.max_around {
                    self.cache.push(index);
                }
            }
        }

        for i in &self.cache {
            self.board[*i] = 0;
        }

        self.cache.len()
    }
}

fn do_day4p1(input: &str, max_around: usize) -> usize {
    Warehouse::new(input, max_around).remove_rolls()
}

fn do_day4p2(input: &str, max_around: usize) -> usize {
    let mut ret = 0;
    let mut warehouse = Warehouse::new(input, max_around);

    loop {
        let last_removed = warehouse.remove_rolls();
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
