use std::collections::HashMap;

fn solve(x: &[i64], count: usize) -> i64 {
    let mut hm = HashMap::new();
    for p in x {
        *hm.entry(*p).or_insert(0) += 1;
    }
    for _ in 0..count {
        let mut nhm = HashMap::new();
        for (s, c) in hm {
            if s == 0 {
                *nhm.entry(1).or_insert(0) += c;
            } else {
                let mut digits = 1;
                let mut counter = 10;
                while s >= counter {
                    counter *= 10;
                    digits += 1;
                }
                if digits % 2 == 0 {
                    let mut div = 1;
                    for _ in 0..digits / 2 {
                        div *= 10;
                    }
                    *nhm.entry(s % div).or_insert(0) += c;
                    *nhm.entry(s / div).or_insert(0) += c;
                } else {
                    *nhm.entry(s * 2024).or_insert(0) += c;
                }
            }
        }
        hm = nhm;
    }
    hm.values().sum()
}

#[aoc(day11, part1)]
pub fn part1(input_struct: &str) -> i64 {
    let q = input_struct
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    solve(&q, 25)
}

#[aoc(day11, part2)]
pub fn part2(input_struct: &str) -> i64 {
    let q = input_struct
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    solve(&q, 75)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "125 17";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 55312);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 81);
    }
}
