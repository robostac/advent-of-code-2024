use std::collections::HashMap;

fn solve(x: &[u64], count: usize) -> u64 {
    let mut hm = HashMap::new();
    let mut nhm = HashMap::new();
    for p in x {
        *nhm.entry(*p).or_insert(0) += 1;
    }
    for _ in 0..count {
        std::mem::swap(&mut hm, &mut nhm);
        nhm.clear();
        for (&s, &c) in hm.iter() {
            if s == 0 {
                *nhm.entry(1).or_insert(0) += c;
            } else {
                let digits = s.ilog10() + 1;
                if digits % 2 == 0 {
                    let div = 10u64.pow(digits / 2);
                    *nhm.entry(s % div).or_insert(0) += c;
                    *nhm.entry(s / div).or_insert(0) += c;
                } else {
                    *nhm.entry(s * 2024).or_insert(0) += c;
                }
            }
        }
    }
    nhm.values().sum()
}

#[aoc(day11, part1)]
pub fn part1(input_struct: &str) -> u64 {
    let q = input_struct
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    solve(&q, 25)
}

#[aoc(day11, part2)]
pub fn part2(input_struct: &str) -> u64 {
    let q = input_struct
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
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
        assert_eq!(part2(TESTLIST), 65601038650482); //not from aoc, calculated manually
    }
}
