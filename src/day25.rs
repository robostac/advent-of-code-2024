#[aoc(day25, part1)]
pub fn part1(input_struct: &str) -> usize {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut k = false;
    let mut vals = [0; 5];

    for l in input_struct.lines() {
        if l.len() == 0 {
            if k {
                keys.push(vals);
            } else {
                locks.push(vals);
            }
            vals = [0; 5];
        } else {
            for (i, c) in l.chars().enumerate() {
                if i == 0 {
                    k = c == '#';
                }
                if c == '#' {
                    vals[i] += 1;
                }
            }
        }
    }
    if k {
        keys.push(vals);
    } else {
        locks.push(vals);
    }

    keys.iter()
        .map(|k| {
            locks
                .iter()
                .filter(|l| (0..5).all(|i| k[i] + l[i] <= 7))
                .count()
        })
        .sum()
}

#[aoc(day25, part2)]
pub fn part2(input_struct: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    const TESTLIST: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    #[test]
    fn sample1() {
        assert_eq!(part1(&TESTLIST), 3);
    }
}
