use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Symbol {
    Do,
    DoNot,
    Mul(i64, i64),
}

type SolutionInput = HashMap<(i64, i64), char>;

#[aoc_generator(day4)]
fn generate(input: &str) -> SolutionInput {
    let mut hm = HashMap::new();
    for (y, s) in input.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            hm.insert((x as i64, y as i64), c);
        }
    }
    hm
}

#[aoc(day4, part1)]
pub fn part1(input: &SolutionInput) -> i64 {
    let mut count = 0;

    let is_mas = |mut p: (i64, i64), dx: i64, dy: i64| {
        for c in ['M', 'A', 'S'] {
            p.0 += dx;
            p.1 += dy;
            if *input.get(&p).unwrap_or(&' ') != c {
                return false;
            }
        }
        true
    };
    for (p, c) in input.iter() {
        if *c == 'X' {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if is_mas(*p, dx, dy) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

const DIAGONALS: [(i64, i64); 4] = [(1, -1), (1, 1), (-1, 1), (-1, -1)];
#[aoc(day4, part2)]
pub fn part2(input: &SolutionInput) -> i64 {
    let mut count = 0;
    for (p, c) in input.iter() {
        if *c == 'A' {
            let mut mas_count = 0;
            for (dx, dy) in DIAGONALS {
                if 'M' == *input.get(&(p.0 + dx, p.1 + dy)).unwrap_or(&' ')
                    && 'S' == *input.get(&(p.0 - dx, p.1 - dy)).unwrap_or(&' ')
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
