use std::ops::*;

fn parse_input(x: &str) -> Vec<i64> {
    x.split(|x| x < '0' || x > '9')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect()
}

fn lcm<
    T: Ord + Rem<Output = T> + PartialEq + Copy + TryFrom<i64> + Mul<Output = T> + Div<Output = T>,
>(
    first: T,
    second: T,
) -> T {
    first * second / gcd(first, second)
}

fn gcd<T: Ord + Rem<Output = T> + PartialEq + Copy + TryFrom<i64>>(first: T, second: T) -> T {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0.try_into().ok().unwrap() {
            return min;
        }

        max = min;
        min = res;
    }
}

fn solve_simple(test: &[i64]) -> Option<(i64, i64, i64)> {
    let mut ba = 0;
    let mut best = None;
    for i in 0..100 {
        let sx = test[0] * i;
        let sy = test[1] * i;
        let rx = test[4] - sx;
        let ry = test[5] - sy;
        if rx >= 0 && rx % test[2] == 0 {
            let bc = rx / test[2];
            let ny = test[3] * bc;
            if ny == ry {
                let cost = 3 * i + bc;
                if best.is_none() || best.unwrap() > cost {
                    best = Some(cost);
                    ba = i;
                }
            }
        }
    }
    if let Some(v) = best {
        Some((v, ba, (test[4] - test[0] * ba) / test[2]))
    } else {
        None
    }
}

fn binary_search_range_max(l: i64, r: i64, res: &dyn Fn(i64) -> bool) -> i64 {
    let mut l = l;
    let mut r = r;
    if res(r) {
        return r;
    }
    while (l + 1) < r {
        let m = l.saturating_add(r) / 2;
        let v = res(m);
        if v == true {
            l = m;
        } else {
            r = m;
        }
    }
    return l;
}

fn solve_fast(test: &[i64]) -> Option<i64> {
    let x1 = test[0];
    let y1 = test[1];
    let x2 = test[2];
    let y2 = test[3];
    let tgtx = test[4];
    let tgty = test[5];
    let diff_x = tgtx % x2;
    let increment_x: i64 = x1 % x2;
    if let Some(cycle_x) = cycle_len(x2, diff_x, increment_x) {
        let diff_y = (tgty - (y1 * cycle_x.0)) % y2;
        let increment_y = (y1 * cycle_x.1) % y2;
        if let Some(cycle_y) = cycle_len(y2, diff_y, increment_y) {
            let count = cycle_x.0 + (cycle_y.0 * cycle_x.1);
            let count_start = count;
            let count_increment = cycle_x.1 * cycle_y.1;
            for slope in [true, false] {
                let i = binary_search_range_max(0, tgtx, &|i| {
                    let new_count = count_start + count_increment * i;
                    let b = (tgtx - new_count * x1) / x2;
                    let ny = y1 * new_count + b * y2;
                    if slope {
                        ny <= tgty
                    } else {
                        ny >= tgty
                    }
                });
                let new_count = count_start + count_increment * i;
                let b = (tgtx - new_count * x1) / x2;
                let cost = 3 * new_count + b;
                if x1 * new_count + x2 * b == tgtx && y1 * new_count + y2 * b == tgty {
                    return Some(cost);
                }
            }
        }
    }
    return None;
}

#[aoc(day13, part1)]
pub fn part1(input_struct: &str) -> i64 {
    let inp = parse_input(input_struct);
    let mut result = 0;
    for test in inp.chunks(6) {
        if let Some(a) = solve_fast(test) {
            result += a;
        }
    }
    result
}

fn cycle_len(group: i64, start: i64, increment: i64) -> Option<(i64, i64)> {
    let mut val: i64 = 0;
    let mut first = -1;

    for i in 0..(2 * group) {
        if val == start {
            if first < 0 {
                first = i;
            } else {
                return Some((first, i - first));
            }
        }
        val = (val + increment) % group;
    }
    return None;
}

#[aoc(day13, part2)]
pub fn part2(input_struct: &str) -> i64 {
    let mut inp = parse_input(input_struct);
    let mut result = 0;
    for test in inp.chunks_mut(6) {
        test[4] += 10000000000000;
        test[5] += 10000000000000;
        if let Some(a) = solve_fast(test) {
            result += a;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 480);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 875318608908);
    }
}
