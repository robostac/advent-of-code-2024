use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let mut ints = input
        .split_ascii_whitespace()
        .enumerate()
        .map(|(i, x)| (i & 1, x.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();
    ints.sort_unstable();
    let z = ints.len() / 2;
    (0..z).map(|x| ints[x].1.abs_diff(ints[z + x].1)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    let mut first = Vec::new();
    let mut second = HashMap::new();
    for x in input.lines() {
        let mut v = x.split_ascii_whitespace();
        first.push(v.next().unwrap().parse::<i64>().unwrap());
        let s = v.next().unwrap().parse::<i64>().unwrap();
        *second.entry(s).or_insert(0) += 1;
    }

    first
        .iter()
        .map(|x| *x * second.get(x).unwrap_or(&0))
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 11);
        assert_eq!(part2(TESTLIST), 31);
    }
}
