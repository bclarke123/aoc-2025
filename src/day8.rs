use std::collections::HashMap;

struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(',').map(|part| part.parse().unwrap());
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let z = parts.next().unwrap();
        Self { x, y, z }
    }

    fn distance(&self, other: &Vector) -> i64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2))
            .abs()
            .isqrt()
    }
}

fn parse_input(input: &str) -> Vec<Vector> {
    input.lines().map(Vector::parse).collect()
}

fn find_distances(input: &[Vector]) -> Vec<(usize, usize)> {
    let mut ret = vec![];

    for a in 0..input.len() {
        for b in 0..input.len() {
            if a == b {
                continue;
            }

            ret.push((a.min(b), a.max(b)));
        }
    }

    ret.sort();
    ret.dedup();

    ret.sort_unstable_by_key(|(a, b)| input[*a].distance(&input[*b]));

    ret
}

struct UnionFind {
    parent: Vec<usize>,
    groups: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            groups: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
            root
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            self.parent[root_i] = root_j;
            self.groups -= 1;
            true
        } else {
            false
        }
    }
}

fn do_day8p1(input: &str, iterations: usize) -> usize {
    let vecs = parse_input(input);
    let pairs = find_distances(&vecs);
    let mut tally = HashMap::<usize, usize>::new();
    let mut uf = UnionFind::new(vecs.len());

    for &(a, b) in pairs.iter().take(iterations) {
        uf.union(a, b);
    }

    for i in 0..vecs.len() {
        let root = uf.find(i);
        *tally.entry(root).or_insert(0) += 1;
    }

    let mut sizes = tally.values().copied().collect::<Vec<_>>();
    sizes.sort_unstable();

    sizes.iter().rev().take(3).product()
}

fn do_day8p2(input: &str) -> i64 {
    let vecs = parse_input(input);
    let pairs = find_distances(&vecs);
    let mut uf = UnionFind::new(vecs.len());
    let mut ret = 0;

    for (a, b) in pairs {
        if uf.union(a, b) && uf.groups == 1 {
            ret = vecs[a].x * vecs[b].x;
            break;
        }
    }

    ret
}

pub fn p1() {
    let input = include_str!("input/day8_input.txt");

    let ret = do_day8p1(input, 1000);

    println!("Day 8 Part 1: The largest circuits multiply to {}", ret);
}

pub fn p2() {
    let input = include_str!("input/day8_input.txt");

    let ret = do_day8p2(input);

    println!(
        "Day 8 Part 2: The final circuit X values multiply to {}",
        ret
    );
}

#[test]
fn test_day8part1() {
    let input = include_str!("input/day8_example.txt");
    let ret = do_day8p1(input, 10);

    assert_eq!(ret, 40);
}

#[test]
fn test_day8part2() {
    let input = include_str!("input/day8_example.txt");
    let ret = do_day8p2(input);

    assert_eq!(ret, 25272);
}

#[test]
fn test_distance() {
    let v1 = Vector { x: 3, y: 4, z: 0 };
    let v2 = Vector { x: 3, y: 4, z: 0 };
    assert_eq!(v1.distance(&v2), 0);

    let v1 = Vector { x: 3, y: 4, z: 0 };
    let v2 = Vector { x: 0, y: 0, z: 0 };
    assert_eq!(v1.distance(&v2), 5);

    let v1 = Vector { x: 1, y: 3, z: 0 };
    let v2 = Vector { x: -2, y: -1, z: 0 };
    assert_eq!(v1.distance(&v2), 5);
}
