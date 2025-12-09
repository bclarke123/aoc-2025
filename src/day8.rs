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
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).abs().isqrt()
    }
}

fn parse_input(input: &str) -> Vec<Vector> {
    input.lines().map(|line| Vector::parse(line)).collect()
}

fn find_distances(input: &Vec<Vector>) -> Vec<(usize, usize)> {
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

fn add(circuit: &mut Vec<usize>, value: usize) {
    if !circuit.contains(&value) {
        circuit.push(value);
    }
}

fn construct_circuits(pairs: &[(usize, usize)])  -> Vec<Vec<usize>> {
    let mut circuits = Vec::<Vec<usize>>::new();

    for (a, b) in pairs {
        let mut added = false;

        'circuit: for i in 0..circuits.len() {
            let circuit = &circuits[i];

            for n in [(a, b), (b, a)] {
                if circuit.contains(n.0) {
                    for x in 0..circuits.len() {
                        if i == x {
                            continue;
                        }

                        if circuits[x].contains(n.1) {
                            let mut cx = circuits.remove(x);
                            circuits[i].append(&mut cx);
                            added = true;

                            break 'circuit;
                        }
                    }

                    add(&mut circuits[i], *n.1);
                    added = true;

                    break 'circuit;
                }
            }
        }

        if !added {
            circuits.push(vec![*a, *b]);
        }
    }

    circuits.sort_unstable_by_key(|circuit| circuit.len());
    circuits.reverse();

    return circuits;
}

fn do_day8p1(input: &str, iterations: usize) -> usize {
    let vecs = parse_input(input);
    let pairs = find_distances(&vecs);
    let closest = &pairs[0..iterations];
    let circuits = construct_circuits(closest);

    circuits[..3].into_iter().fold(1, |a, c| a * c.len())
}

pub fn p1() {
    let input = include_str!("input/day8_input.txt");

    let ret = do_day8p1(input, 1000);

    println!("Day 8 Part 1: The largest circuits multiply to {}", ret);
}

#[test]
fn test_day8part1() {
    let input = include_str!("input/day8_example.txt");
    let ret = do_day8p1(input, 10);

    assert_eq!(ret, 40);
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
