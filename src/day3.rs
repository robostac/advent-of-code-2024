#[derive(Debug, Clone)]
pub enum Symbol {
    Do,
    DoNot,
    Mul(i64, i64),
}
#[aoc_generator(day3)]
fn symbols(input: &str) -> Vec<Symbol> {
    let mut start = None;
    let mut d = false;
    let mut sym = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if c == 'm' {
            start = Some(i);
            d = false;
        } else if c == 'd' {
            start = Some(i);
            d = true;
        }
        if c == ')' && start.is_some() {
            let s = start.unwrap();
            if d {
                if &input[s..i] == "do(" {
                    sym.push(Symbol::Do);
                }
                if &input[s..i] == "don't(" {
                    sym.push(Symbol::DoNot);
                }
            } else if s + 4 <= i {
                if &input[s..s + 4] == "mul(" {
                    if let Some((a, b)) = input[s + 4..i].split_once(',') {
                        if let Some(aa) = a.parse::<i64>().ok() {
                            if let Some(bb) = b.parse::<i64>().ok() {
                                sym.push(Symbol::Mul(aa, bb));
                            }
                        }
                    }
                }
            }
            start = None;
        }
    }
    sym
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Symbol>) -> i64 {
    input
        .iter()
        .map(|x| if let Symbol::Mul(a, b) = x { a * b } else { 0 })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Symbol>) -> i64 {
    let mut enabled = 1;
    input
        .iter()
        .map(|x| match x {
            Symbol::Mul(a, b) => enabled * a * b,
            Symbol::Do => {
                enabled = 1;
                0
            }
            Symbol::DoNot => {
                enabled = 0;
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, symbols};

    const TESTLIST: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    #[test]
    fn sample1() {
        assert_eq!(part1(&symbols(TESTLIST)), 161);
    }
    const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn sample2() {
        assert_eq!(part2(&symbols(TEST2)), 48);
    }
}
