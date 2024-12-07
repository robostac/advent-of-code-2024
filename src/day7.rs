fn possible(ans: i64, vals: &[i64]) -> bool {
    let l = *vals.last().unwrap();
    if vals.len() == 1 {
        return ans == l;
    }
    if ans < 0 {
        return false;
    }

    return possible(ans - l, &vals[..vals.len() - 1])
        || (ans % l == 0 && possible(ans / l, &vals[..vals.len() - 1]));
}

#[aoc(day7, part1)]
pub fn part1(input_struct: &str) -> i64 {
    let mut result = 0;
    for inp in input_struct.lines() {
        let (ans, values) = inp.split_once(':').unwrap();
        let ans = ans.parse::<i64>().unwrap();
        let values = values
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        if possible(ans, &values) {
            result += ans;
        }
    }
    result
}

fn possible2(ans: i64, vals: &[i64]) -> bool {
    let l = *vals.last().unwrap();
    if vals.len() == 1 {
        return ans == l;
    }
    if ans < 0 {
        return false;
    }
    let mut valid = false;

    valid = valid || possible2(ans - l, &vals[..vals.len() - 1]);
    valid = valid || (ans % l == 0 && possible2(ans / l, &vals[..vals.len() - 1]));
    if valid == false && vals.len() >= 2 {
        let mut test = 1;
        while test <= l {
            test *= 10;
        }
        if ans % test == l {
            valid = possible2(ans / test, &vals[..vals.len() - 1]);
        }
    }
    valid
}

#[aoc(day7, part2)]
pub fn part2(input_struct: &str) -> i64 {
    let mut result = 0;
    for inp in input_struct.lines() {
        let (ans_s, values) = inp.split_once(':').unwrap();
        let ans = ans_s.parse::<i64>().unwrap();
        let values = values
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        if possible2(ans, &values) {
            result += ans;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 3749);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 11387);
    }
}
