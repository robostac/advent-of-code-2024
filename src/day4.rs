#[derive(Debug, Clone)]
pub enum Symbol {
    Do,
    DoNot,
    Mul(i64, i64),
}

// type SolutionInput = HashMap<(i64, i64), char>;
type SolutionInput = (Vec<char>, i64);

#[aoc_generator(day4)]
fn generate(input: &str) -> SolutionInput {
    let cc: Vec<char> = input.chars().collect();
    let line = cc.iter().position(|x| *x == '\n').unwrap() as i64 + 1;
    (cc, line)
}

#[aoc(day4, part1)]
pub fn part1(input: &SolutionInput) -> i64 {
    let mut count = 0;

    let is_mas = |mut p: i64, dp: i64| {
        for c in ['M', 'A', 'S'] {
            p += dp;
            if p < 0 || *input.0.get(p as usize).unwrap_or(&' ') != c {
                return false;
            }
        }
        true
    };
    for (p, c) in input.0.iter().enumerate() {
        if *c == 'X' {
            for d in [
                1,
                -1,
                input.1,
                -input.1,
                input.1 + 1,
                input.1 - 1,
                -input.1 + 1,
                -input.1 - 1,
            ] {
                if is_mas(p as i64, d) {
                    count += 1;
                }
            }
        }
    }
    count
}

#[aoc(day4, part2)]
pub fn part2(input: &SolutionInput) -> i64 {
    let mut count = 0;
    for (p, c) in input.0.iter().enumerate() {
        if *c == 'A' {
            let mut mas_count = 0;
            for d in [input.1 + 1, input.1 - 1, -input.1 + 1, -input.1 - 1] {
                if 'M' == *input.0.get((p as i64 + d) as usize).unwrap_or(&' ')
                    && 'S' == *input.0.get((p as i64 - d) as usize).unwrap_or(&' ')
                {
                    mas_count += 1;
                }
            }
            if mas_count == 2 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::{generate, part1, part2};

    const TESTLIST: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    #[test]
    fn sample1() {
        assert_eq!(part1(&generate(TESTLIST)), 18);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&generate(TESTLIST)), 9);
    }
}
