#[aoc(day4, part1)]
pub fn part1(input_struct: &str) -> i64 {
    let input: Vec<char> = input_struct.chars().collect();
    let line_length = input.iter().position(|x| *x == '\n').unwrap() as i64 + 1;
    let mut count = 0;

    let is_mas = |mut p: i64, dp: i64| {
        for c in ['M', 'A', 'S'] {
            p += dp;
            if p < 0 || *input.get(p as usize).unwrap_or(&' ') != c {
                return false;
            }
        }
        true
    };
    for (p, c) in input.iter().enumerate() {
        if *c == 'X' {
            for d in [
                1,
                -1,
                line_length,
                -line_length,
                line_length + 1,
                line_length - 1,
                -line_length + 1,
                -line_length - 1,
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
pub fn part2(input_struct: &str) -> i64 {
    let input: Vec<char> = input_struct.chars().collect();
    let line_length = input.iter().position(|x| *x == '\n').unwrap() as i64 + 1;
    let mut count = 0;
    for (p, c) in input.iter().enumerate() {
        if *c == 'A' {
            let mut mas_count = 0;
            for d in [
                line_length + 1,
                line_length - 1,
                -line_length + 1,
                -line_length - 1,
            ] {
                if 'M' == *input.get((p as i64 + d) as usize).unwrap_or(&' ')
                    && 'S' == *input.get((p as i64 - d) as usize).unwrap_or(&' ')
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
    use super::{part1, part2};

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
        assert_eq!(part1(TESTLIST), 18);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 9);
    }
}
