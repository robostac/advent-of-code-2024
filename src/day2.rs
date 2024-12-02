fn safe(x: &[i32]) -> Option<usize> {
    let diff = (x[1] - x[0]).signum();
    if diff == 0 {
        return Some(0);
    }
    for (i, pair) in x.windows(2).enumerate() {
        let v = pair[1] - pair[0];
        if v.abs() > 3 || v.signum() != diff {
            return Some(i);
        }
    }
    None
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let v = l
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            safe(&v).is_none()
        })
        .count()
}

fn safe_skip(x: &Vec<i32>, skip: usize) -> bool {
    let mut idx = 1;
    let mut last = x[0];
    if skip == 0 {
        return safe(&x[1..]).is_none();
    } else if skip == 1 {
        idx = 2;
    }
    let diff = (x[idx] - x[0]).signum();
    if diff == 0 {
        return false;
    }
    for (i, &val) in x.iter().enumerate().skip(1) {
        if i == skip {
            continue;
        }
        let v = val - last;
        last = val;
        if v.abs() > 3 || v.signum() != diff {
            return false;
        }
    }
    true
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let x = l
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            match safe(&x) {
                None => true,
                Some(idx) => (idx.saturating_sub(1)..idx + 2).any(|y| safe_skip(&x, y)),
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 2);
        assert_eq!(part2(TESTLIST), 4);
    }
}
