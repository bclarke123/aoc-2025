use std::collections::HashMap;

#[derive(Debug)]
struct Board {
    start: usize,
    width: usize,
    splitters: Vec<bool>,
}

impl Board {
    fn new(input: &str) -> Self {
        let mut chars = input.lines();
        let start_line = chars.by_ref().next().unwrap();

        let start = start_line.chars().position(|c| c == 'S').unwrap();
        let width = start_line.len();

        let splitters = chars
            .flat_map(|line| line.chars().map(|c| c == '^').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            start,
            width,
            splitters,
        }
    }

    fn height(&self) -> usize {
        self.splitters.len() / self.width
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn is_splitter(&self, x: usize, y: usize) -> bool {
        self.splitters[self.index(x, y)]
    }

    fn add(&self, beam: &mut Vec<usize>, index: usize) {
        if !beam.contains(&index) {
            beam.push(index);
        }
    }

    fn solve(&self) -> u64 {
        let height = self.height();
        let mut beams = vec![self.start];
        let mut ret = 0;

        for y in 0..height {
            let n_beams = beams.len();

            for _ in 0..n_beams {
                let beam = beams.remove(0);
                if self.is_splitter(beam, y) {
                    ret += 1;
                    self.add(&mut beams, beam + 1);
                    self.add(&mut beams, beam - 1);
                } else {
                    self.add(&mut beams, beam);
                }
            }
        }

        ret
    }

    fn path(&self, x: usize, y: usize, cache: &mut HashMap<usize, u64>) -> u64 {
        let height = self.height();
        let next_y = y + 1;

        if next_y >= height {
            return 1;
        }

        let index = self.index(x, y);
        if !cache.contains_key(&index) {
            let ret = if self.is_splitter(x, y) {
                self.path(x - 1, next_y, cache) + self.path(x + 1, next_y, cache)
            } else {
                self.path(x, next_y, cache)
            };

            cache.insert(index, ret);
        }

        cache[&index]
    }

    fn solve_p2(&self) -> u64 {
        self.path(self.start, 0, &mut HashMap::new())
    }
}

fn do_day7p1(input: &str) -> u64 {
    Board::new(input).solve()
}

fn do_day7p2(input: &str) -> u64 {
    Board::new(input).solve_p2()
}

pub fn p1() {
    let input = include_str!("input/day7_input.txt");

    let ret = do_day7p1(input);

    println!("Day 7 Part 1: The beam splits {} times", ret);
}

pub fn p2() {
    let input = include_str!("input/day7_input.txt");

    let ret = do_day7p2(input);

    println!("Day 7 Part 2: The beam encounters {} universes", ret);
}

#[test]
fn test_day7p1() {
    let input = include_str!("input/day7_example.txt");

    let ret = do_day7p1(input);
    assert_eq!(ret, 21);
}

#[test]
fn test_day7p2() {
    let input = include_str!("input/day7_example.txt");

    let ret = do_day7p2(input);
    assert_eq!(ret, 40);
}
