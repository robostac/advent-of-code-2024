use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ACTIVE,
}

impl Direction {
    fn pos(&self) -> (i32, i32) {
        match self {
            Direction::ACTIVE => (2, 0),
            Direction::DOWN => (1, 1),
            Direction::UP => (1, 0),
            Direction::RIGHT => (2, 1),
            Direction::LEFT => (0, 1),
        }
    }
}

fn push_button(
    layer: usize,
    bttn: Direction,
    current: Direction,
    dp: &mut HashMap<(usize, Direction, Direction), usize>,
) -> usize {
    if layer == 0 {
        return 1;
    }
    let k = (layer, bttn, current);
    if let Some(v) = dp.get(&k) {
        return *v;
    }
    let vert_first = {
        let mut dist = 0;
        let curpos = current.pos();
        let tgtpos = bttn.pos();
        let dx: i32 = tgtpos.0 - curpos.0;
        let dy: i32 = tgtpos.1 - curpos.1;
        let mut prev_but = Direction::ACTIVE;
        if tgtpos.1 == 0 && curpos.0 == 0 {
            None
        } else {
            for _ in 0..dy.abs() {
                if dy < 0 {
                    dist += push_button(layer - 1, Direction::UP, prev_but, dp);
                    prev_but = Direction::UP;
                } else {
                    dist += push_button(layer - 1, Direction::DOWN, prev_but, dp);
                    prev_but = Direction::DOWN;
                }
            }
            for _ in 0..dx.abs() {
                if dx < 0 {
                    dist += push_button(layer - 1, Direction::LEFT, prev_but, dp);
                    prev_but = Direction::LEFT;
                } else {
                    dist += push_button(layer - 1, Direction::RIGHT, prev_but, dp);
                    prev_but = Direction::RIGHT;
                }
            }
            dist += push_button(layer - 1, Direction::ACTIVE, prev_but, dp);
            Some(dist)
        }
    };
    let horiz_first = {
        let mut dist = 0;
        let curpos = current.pos();
        let tgtpos = bttn.pos();
        let dx: i32 = tgtpos.0 - curpos.0;
        let dy: i32 = tgtpos.1 - curpos.1;
        let mut prev_but = Direction::ACTIVE;
        if tgtpos.0 == 0 && curpos.1 == 0 {
            None
        } else {
            for _ in 0..dx.abs() {
                if dx < 0 {
                    dist += push_button(layer - 1, Direction::LEFT, prev_but, dp);
                    prev_but = Direction::LEFT;
                } else {
                    dist += push_button(layer - 1, Direction::RIGHT, prev_but, dp);
                    prev_but = Direction::RIGHT;
                }
            }
            for _ in 0..dy.abs() {
                if dy < 0 {
                    dist += push_button(layer - 1, Direction::UP, prev_but, dp);
                    prev_but = Direction::UP;
                } else {
                    dist += push_button(layer - 1, Direction::DOWN, prev_but, dp);
                    prev_but = Direction::DOWN;
                }
            }
            dist += push_button(layer - 1, Direction::ACTIVE, prev_but, dp);

            Some(dist)
        }
    };
    let val;
    if horiz_first.is_none() {
        val = vert_first.unwrap()
    } else if vert_first.is_none() {
        val = horiz_first.unwrap();
    } else {
        val = vert_first.unwrap().min(horiz_first.unwrap());
    }
    dp.insert(k, val);
    val
}

fn solve(input_struct: &str, first_layer: usize) -> usize {
    let pos = [
        (1, 3),
        (0, 2),
        (1, 2),
        (2, 2),
        (0, 1),
        (1, 1),
        (2, 1),
        (0, 0),
        (1, 0),
        (2, 0),
        (2, 3),
    ];
    let mut ans = 0;
    let mut hm = HashMap::new();
    for l in input_struct.lines() {
        let mut dist = 0;

        let mut curpos = pos[0xa];
        for p in l.chars() {
            let d = p.to_digit(16).unwrap() as usize;
            let dx: i32 = pos[d].0 - curpos.0;
            let dy: i32 = pos[d].1 - curpos.1;
            let vert_first = {
                let mut dist = 0;
                let mut cur_button = Direction::ACTIVE;
                if pos[d].1 == 3 && curpos.0 == 0 {
                    None
                } else {
                    for _ in 0..dy.abs() {
                        if dy < 0 {
                            dist += push_button(first_layer, Direction::UP, cur_button, &mut hm);
                            cur_button = Direction::UP;
                        } else {
                            dist += push_button(first_layer, Direction::DOWN, cur_button, &mut hm);
                            cur_button = Direction::DOWN;
                        }
                    }

                    for _ in 0..dx.abs() {
                        if dx < 0 {
                            dist += push_button(first_layer, Direction::LEFT, cur_button, &mut hm);
                            cur_button = Direction::LEFT;
                        } else {
                            dist += push_button(first_layer, Direction::RIGHT, cur_button, &mut hm);
                            cur_button = Direction::RIGHT;
                        }
                    }
                    // println!("\t {} {:?} {:?}", p, dist, cur_button);
                    dist += push_button(first_layer, Direction::ACTIVE, cur_button, &mut hm);
                    Some(dist)
                }
            };
            let horiz_first = {
                let mut dist = 0;
                let mut cur_button = Direction::ACTIVE;
                if curpos.1 == 3 && pos[d].0 == 0 {
                    None
                } else {
                    for _ in 0..dx.abs() {
                        if dx < 0 {
                            dist += push_button(first_layer, Direction::LEFT, cur_button, &mut hm);
                            cur_button = Direction::LEFT;
                        } else {
                            dist += push_button(first_layer, Direction::RIGHT, cur_button, &mut hm);
                            cur_button = Direction::RIGHT;
                        }
                    }
                    for _ in 0..dy.abs() {
                        if dy < 0 {
                            dist += push_button(first_layer, Direction::UP, cur_button, &mut hm);
                            cur_button = Direction::UP;
                        } else {
                            dist += push_button(first_layer, Direction::DOWN, cur_button, &mut hm);
                            cur_button = Direction::DOWN;
                        }
                    }

                    // println!("\t {} {:?} {:?}", p, dist, cur_button);
                    dist += push_button(first_layer, Direction::ACTIVE, cur_button, &mut hm);
                    Some(dist)
                }
            };
            let val;
            if horiz_first.is_none() {
                val = vert_first.unwrap()
            } else if vert_first.is_none() {
                val = horiz_first.unwrap();
            } else {
                val = vert_first.unwrap().min(horiz_first.unwrap());
            }
            dist += val;
            curpos = pos[d];
        }
        let va = l[..l.len() - 1].parse::<usize>().unwrap();

        ans += va * dist;
    }
    ans
}

#[aoc(day21, part1)]
pub fn part1(input_struct: &str) -> usize {
    solve(input_struct, 2)
}

#[aoc(day21, part2)]
pub fn part2(input_struct: &str) -> usize {
    solve(input_struct, 25)
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    const TESTLIST: &str = "029A
980A
179A
456A
379A";
    #[test]
    fn sample1() {
        assert_eq!(part1(&TESTLIST), 126384);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&TESTLIST), 16);
    }
}
