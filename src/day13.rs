fn parse_input(x: &str) -> Vec<i64> {
    x.split(|x| x < '0' || x > '9')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect()
}

fn solve_instant(test: &[i64]) -> Option<i64> {
    let mut x1 = test[0];
    let y1 = test[1];
    let mut x2 = test[2];
    let mut y2 = test[3];
    let mut tgtx = test[4];
    let mut tgty = test[5];

    let m1 = y1;
    let m2 = x1;

    x1 *= m1;
    x2 *= m1;
    tgtx *= m1;

    y2 *= m2;
    tgty *= m2;

    //tgty - b * y2 == tgtx - b * y1
    // b(y2 - y1) = tgty - tgtx

    let dy = y2 - x2;
    let dt = tgty - tgtx;

    if dt % dy == 0 {
        let b = dt / dy;
        let da = tgtx - b * x2;

        if da % x1 == 0 {
            return Some((da / x1) * 3 + b);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

#[aoc(day13, part1)]
pub fn part1(input_struct: &str) -> i64 {
    let inp = parse_input(input_struct);
    let mut result = 0;
    for test in inp.chunks(6) {
        if let Some(a) = solve_instant(test) {
            result += a;
        }
    }
    result
}

#[aoc(day13, part2)]
pub fn part2(input_struct: &str) -> i64 {
    let mut inp = parse_input(input_struct);
    let mut result = 0;
    for test in inp.chunks_mut(6) {
        test[4] += 10000000000000;
        test[5] += 10000000000000;
        if let Some(a) = solve_instant(test) {
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
