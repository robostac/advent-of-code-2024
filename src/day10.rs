#[aoc(day10, part1)]
pub fn part1(input_struct: &str) -> i64 {
    let line_length = input_struct.chars().position(|x| x == '\n').unwrap() as i64 + 1;
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    let input = input_struct
        .chars()
        .enumerate()
        .map(|(i, x)| {
            if let Some(p) = x.to_digit(10) {
                if p == 0 {
                    starts.push(i);
                } else if p == 9 {
                    ends.push(i);
                }
                p as i8
            } else {
                -2
            }
        })
        .collect::<Vec<_>>();
    let mut ans = 0;
    let mut reachable = vec![usize::MAX; input.len()];
    let mut current = Vec::new();
    for (i, &e) in ends.iter().enumerate() {
        current.push(e);
        while let Some(c) = current.pop() {
            let v = input[c];
            let tgt = v - 1;
            let posi64 = c as i64;
            for next_pos in [
                posi64 + 1,
                posi64 - 1,
                posi64 + line_length,
                posi64 - line_length,
            ] {
                if let Some(new_v) = input.get(next_pos as usize) {
                    if *new_v == tgt && reachable[next_pos as usize] != i {
                        current.push(next_pos as usize);
                        reachable[next_pos as usize] = i;
                    }
                }
            }
        }
        for s in starts.iter() {
            if reachable[*s] == i {
                ans += 1;
            }
        }
    }
    ans
}

fn distinct(inp: &[i8], pos: usize, dp: &mut [i64], width: i64) -> i64 {
    let current = inp[pos];
    if current == 9 {
        return 1;
    }
    if dp[pos] < 0 {
        dp[pos] = 0;
        let posi64 = pos as i64;
        for next_pos in [posi64 + 1, posi64 - 1, posi64 + width, posi64 - width] {
            if let Some(next_val) = inp.get(next_pos as usize) {
                if *next_val == current + 1 {
                    dp[pos] += distinct(inp, next_pos as usize, dp, width);
                }
            }
        }
    }
    dp[pos as usize]
}

#[aoc(day10, part2)]
pub fn part2(input_struct: &str) -> i64 {
    let line_length = input_struct.chars().position(|x| x == '\n').unwrap() as i64 + 1;
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    let input = input_struct
        .chars()
        .enumerate()
        .map(|(i, x)| {
            if let Some(p) = x.to_digit(10) {
                if p == 0 {
                    starts.push(i);
                } else if p == 9 {
                    ends.push(i);
                }
                p as i8
            } else {
                -1
            }
        })
        .collect::<Vec<_>>();
    let mut ans = 0;
    let mut dp = vec![-1; input.len()];

    for s in starts.iter() {
        ans += distinct(&input, *s, &mut dp, line_length);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 36);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 81);
    }
}
