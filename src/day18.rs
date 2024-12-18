use std::collections::VecDeque;

fn parse_in(s: &str) -> Vec<(i64, i64)> {
    s.lines()
        .map(|x| {
            let (a, b) = x.split_once(',').unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect()
}

fn solve(width: i64, height: i64, bytes: &[(i64, i64)]) -> Option<i64> {
    let mut bad = vec![vec![i64::MAX; height as usize]; width as usize];
    let mut vdq = VecDeque::new();
    vdq.push_back((0i64, 0i64));

    bad[0][0] = 0;
    for (x, y) in bytes.iter() {
        bad[*x as usize][*y as usize] = -1;
    }

    while let Some((x, y)) = vdq.pop_front() {
        let t = bad[x as usize][y as usize];

        if t < 0 {
            continue;
        }

        for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if nx < 0 || nx >= height || ny < 0 || ny >= height {
                continue;
            }
            let z = bad[nx as usize][ny as usize];
            if z <= t + 1 {
                continue;
            }
            bad[nx as usize][ny as usize] = t + 1;
            vdq.push_back((nx, ny));
        }
    }
    if bad[width as usize - 1][height as usize - 1] == i64::MAX {
        return None;
    }
    Some(bad[width as usize - 1][height as usize - 1])
}

#[aoc(day18, part1)]
pub fn part1(input_struct: &str) -> i64 {
    solve(71, 71, &parse_in(input_struct)[..1024]).unwrap()
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

fn solve2(width: i64, height: i64, bytes: &[(i64, i64)]) -> String {
    let ans = binary_search_range_max(0, bytes.len() as i64, &|x| {
        solve(width, height, &bytes[..(x as usize)]).is_some()
    });
    format!("{},{}", bytes[ans as usize].0, bytes[ans as usize].1)
}
#[aoc(day18, part2)]
pub fn part2(input_struct: &str) -> String {
    let b = &parse_in(input_struct);
    solve2(71, 71, &b)
}

#[cfg(test)]
mod tests {
    use crate::day18::parse_in;

    use super::{solve, solve2};

    const TESTLIST: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    #[test]
    fn sample1() {
        assert_eq!(solve(7, 7, &parse_in(&TESTLIST)[..12]), Some(22));
    }

    #[test]
    fn sample2() {
        assert_eq!(solve2(7, 7, &parse_in(&TESTLIST)), "6,1");
    }
}
