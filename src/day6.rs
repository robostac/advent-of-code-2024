use std::usize;

#[aoc(day6, part1)]
pub fn part1(input_struct: &str) -> usize {
    let mut c = input_struct.chars().collect::<Vec<_>>();
    let width = c.iter().position(|x| *x == '\n').unwrap() as i64 + 1;
    let pos = c.iter().position(|x| *x == '^').unwrap() as i64;
    let directions = [-width, 1, width, -1];
    let dir = 0;
    c[pos as usize] = '.';
    let mut seen = vec![0; input_struct.len()];
    cycle_find(&c, &directions, &mut seen, pos, dir);
    seen.iter().filter(|x| **x > 0).count()
}

fn cycle_find(
    c: &[char],
    directions: &[i64],
    seen: &mut [usize],
    mut pos: i64,
    mut dir: i64,
) -> bool {
    loop {
        let np = directions[dir as usize] + pos;
        let v = *c.get(np as usize).unwrap_or(&'\n');

        if v == '\n' {
            return false;
        } else if v == '#' {
            dir = (dir + 1) & 3;
        } else {
            pos = np;
            if seen[np as usize] & (1 << dir) > 0 {
                return true;
            }
            seen[np as usize] |= 1 << dir;
        }
    }
}

#[aoc(day6, part2)]
pub fn part2(input_struct: &str) -> usize {
    let mut c = input_struct.chars().collect::<Vec<_>>();
    let width = c.iter().position(|x| *x == '\n').unwrap() as i64 + 1;
    let mut pos = c.iter().position(|x| *x == '^').unwrap() as i64;
    let directions = [-width, 1, width, -1];
    let mut dir = 0;
    c[pos as usize] = '.';
    let mut seen = vec![0; input_struct.len()];
    let mut count = 0;
    loop {
        let np = directions[dir as usize] + pos;
        if np < 0 {
            break;
        }
        let v = *c.get(np as usize).unwrap_or(&'\n');

        if v == '\n' {
            break;
        } else if v == '#' {
            dir = (dir + 1) & 3;
        } else {
            if seen[np as usize] == 0 {
                let mut ss = seen.clone();
                c[np as usize] = '#';
                if cycle_find(&c, &directions, &mut ss, pos, dir) {
                    count += 1;
                }
                c[np as usize] = '.';
            }
            pos = np;
            seen[pos as usize] |= 1 << dir;
        }
    }
    // cycle_find(&c, &directions, &mut seen, pos, dir);
    count
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 41);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 6);
    }
}
