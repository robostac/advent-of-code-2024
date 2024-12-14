fn parse_input(x: &str) -> Vec<i64> {
    x.split(|x| (x < '0' || x > '9') && x != '-')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect()
}

fn check_pos(p: i64, m: i64) -> Option<i64> {
    if p == m {
        return None;
    }
    if p < m {
        return Some(0);
    } else {
        return Some(1);
    }
}

fn solve_p1(robots: &[i64], width: i64, height: i64) -> i64 {
    let mut quadrants = [0; 4];
    let mx = width / 2;
    let my = height / 2;
    for r in robots.chunks(4) {
        let sx = r[0];
        let sy = r[1];
        let vx = r[2];
        let vy = r[3];

        let ex = (((sx + (vx * 100)) % width) + width) % width;
        let ey = (((sy + (vy * 100)) % height) + height) % height;

        if let Some(qx) = check_pos(ex, mx) {
            if let Some(qy) = check_pos(ey, my) {
                quadrants[(qx + 2 * qy) as usize] += 1;
            }
        }
    }
    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

#[aoc(day14, part1)]
pub fn part1(input_struct: &str) -> i64 {
    let inp = parse_input(input_struct);
    solve_p1(&inp, 101, 103)
}

#[aoc(day14, part2)]
pub fn part2(input_struct: &str) -> i64 {
    let mut inp = parse_input(input_struct);
    let width = 101i64;
    let height = 103i64;
    let frame_line = 26;
    for t in 1.. {
        let mut frame = 0u128;
        for r in inp.chunks_mut(4) {
            let sx = r[0];
            let sy = r[1];
            let vx = r[2];
            let vy = r[3];

            let ex = (((sx + vx) % width) + width) % width;
            let ey = (((sy + vy) % height) + height) % height;
            r[0] = ex;
            r[1] = ey;
            if ey == frame_line {
                frame |= 1 << ex;
            }
        }
        if frame & 0xfffffffe000000000 == 0xfffffffe000000000 {
            return t;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2, solve_p1};

    const TESTLIST: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
    #[test]
    fn sample1() {
        let inp = parse_input(TESTLIST);
        assert_eq!(solve_p1(&inp, 11, 7), 12);
    }
}
