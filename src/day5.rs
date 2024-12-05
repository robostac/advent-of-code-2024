fn valid(z: &[usize], bad_mask: &[u128]) -> Option<usize> {
    let mut bad = 0;

    for (i, &p) in z.iter().enumerate() {
        if bad & (1 << p) > 0 {
            return Some(i);
        }
        bad |= bad_mask[p];
    }

    None
}

#[aoc(day5, part1)]
pub fn part1(input_struct: &str) -> usize {
    let mut bad_mask = [0u128; 100];
    let mut ans = 0;
    for ln in input_struct.lines() {
        if let Some((a, b)) = ln.split_once("|") {
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();

            bad_mask[b] |= 1 << a;
        } else if ln.len() > 0 {
            let z = ln
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if valid(&z, &bad_mask).is_none() {
                ans += z[z.len() / 2];
            }
        }
    }
    ans
}

#[aoc(day5, part2)]
pub fn part2(input_struct: &str) -> usize {
    let mut bad_mask = [0u128; 100];
    let mut ans = 0;
    for ln in input_struct.lines() {
        if let Some((a, b)) = ln.split_once("|") {
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();

            bad_mask[b] |= 1 << a;
        } else if ln.len() > 0 {
            let mut z = ln
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let mut changed = false;
            while let Some(p) = valid(&z, &bad_mask) {
                let v = z.remove(p);
                z.insert(0, v);
                changed = true;
            }
            if changed {
                ans += z[z.len() / 2];
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 143);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 123);
    }
}
