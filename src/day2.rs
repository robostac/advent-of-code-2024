#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn safe(x: &Vec<i32>) -> Option<usize> {
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
pub fn part1(input: &Vec<Vec<i32>>) -> usize {
    input.iter().filter(|x| safe(&x).is_none()).count()
}

fn safe_skip(x: &Vec<i32>, skip: usize) -> bool {
    let mut idx = 1;
    let mut last = x[0];
    if skip == 0 {
        idx += 1;
        last = x[1];
    }
    let diff = (x[idx] - last).signum();
    if diff == 0 {
        return false;
    }
    loop {
        if idx == skip {
            idx += 1;
        }
        if idx >= x.len() {
            return true;
        }

        let v = x[idx] - last;
        last = x[idx];
        if v.abs() > 3 || v.signum() != diff {
            return false;
        }
        idx += 1;
    }
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Vec<i32>>) -> usize {
    input
        .iter()
        .filter(|&x| match safe(x) {
            None => true,
            Some(idx) => (idx.saturating_sub(1)..idx + 2).any(|y| safe_skip(&x, y)),
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const TESTLIST: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(TESTLIST)), 2);
        assert_eq!(part2(&input_generator(TESTLIST)), 4);
    }
}
