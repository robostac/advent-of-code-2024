#[aoc(day22, part1)]
pub fn part1(input_struct: &str) -> i64 {
    let mut ans = 0;
    for p in input_struct.split_ascii_whitespace() {
        let mut p = p.parse::<i64>().unwrap();
        for _ in 0..2000 {
            p = next(p);
        }
        ans += p;
    }
    ans
}

fn next(mut x: i64) -> i64 {
    x = ((x * 64) ^ x) % 16777216;
    x = ((x / 32) ^ x);
    x = ((x * 2048) ^ x) % 16777216;
    x
}

#[aoc(day22, part2)]
pub fn part2(input_struct: &str) -> i64 {
    let mut ans = vec![0; 1 << 20];
    let mut seen = vec![0; 1 << 20];
    let mut iter = 0;
    for p in input_struct.split_ascii_whitespace() {
        iter += 1;
        let mut p = p.parse::<i64>().unwrap();

        let mut last = p % 10;
        let mut seq = 0;
        for i in 1..2000 {
            p = next(p);
            let diff = (p % 10) - last;
            seq = ((seq & 0x7FFF) << 5) + (diff + 9);
            last = p % 10;
            if i >= 4 {
                let s = seq as usize;
                if seen[s] != iter {
                    ans[s] += last;
                    seen[s] = iter;
                }
            }
        }
    }
    *ans.iter().max().unwrap()
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    const TESTLIST: &str = "
    1
10
100
2024";
    #[test]
    fn sample1() {
        assert_eq!(part1(&TESTLIST), 37327623);
    }
    const TESTLIST2: &str = "
    1
2
3
2024";
    #[test]
    fn sample2() {
        assert_eq!(part2(&TESTLIST2), 23);
    }
}
