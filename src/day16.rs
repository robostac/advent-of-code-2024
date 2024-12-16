use std::collections::VecDeque;
fn do_move(p: usize, d: i64) -> usize {
    (p as i64 + d) as usize
}

#[aoc(day16, part1)]
pub fn part1(input_struct: &str) -> usize {
    let mut input: Vec<char> = input_struct.chars().collect();
    let line_length = input.iter().position(|x| *x == '\n').unwrap() as i64 + 1;
    let start = input.iter().position(|x| *x == 'S').unwrap();
    let end = input.iter().position(|x| *x == 'E').unwrap();

    input[start] = '.';
    input[end] = '.';
    let directions = [1, line_length, -1, -line_length];
    let mut scores = vec![[usize::MAX; 4]; input.len()];
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    scores[start][0] = 0;
    while let Some((p, d)) = q.pop_front() {
        let cur_score = scores[p][d];
        for (np, nd, ns) in [
            (do_move(p, directions[d]), d, 1),
            (p, (d + 1) % 4, 1000),
            (p, (d + 3) % 4, 1000),
        ] {
            if input[np] == '.' {
                if cur_score + ns < scores[np][nd] {
                    q.push_back((np, nd));
                    scores[np][nd] = cur_score + ns;
                }
            }
        }
    }
    *scores[end].iter().min().unwrap()
}

struct Maze<'a> {
    scores: &'a [[usize; 4]],
    input: &'a [char],
    directions: &'a [i64],
}

impl Maze<'_> {
    fn dfs(&self, on_path: &mut [[bool; 4]], pos: usize, d: usize, end: usize, cur: usize) -> bool {
        if self.scores[pos][d] < cur {
            return false;
        }
        if self.input[pos] == '#' {
            return false;
        }
        if on_path[pos][d] {
            return true;
        }
        if pos == end {
            on_path[pos][d] = true;
            return true;
        }
        for (np, nd, ns) in [
            (do_move(pos, self.directions[d]), d, 1),
            (pos, (d + 1) % 4, 1000),
            (pos, (d + 3) % 4, 1000),
        ] {
            on_path[pos][d] |= self.dfs(on_path, np, nd, end, cur + ns)
        }
        return on_path[pos][d];
    }
}
#[aoc(day16, part2)]
pub fn part2(input_struct: &str) -> usize {
    let mut input: Vec<char> = input_struct.chars().collect();
    let line_length = input.iter().position(|x| *x == '\n').unwrap() as i64 + 1;
    let start = input.iter().position(|x| *x == 'S').unwrap();
    let end = input.iter().position(|x| *x == 'E').unwrap();

    input[start] = '.';
    input[end] = '.';
    let directions = [1, line_length, -1, -line_length];
    let mut scores = vec![[usize::MAX; 4]; input.len()];
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    scores[start][0] = 0;
    while let Some((p, d)) = q.pop_front() {
        let cur_score = scores[p][d];
        for (np, nd, ns) in [
            (do_move(p, directions[d]), d, 1),
            (p, (d + 1) % 4, 1000),
            (p, (d + 3) % 4, 1000),
        ] {
            if input[np] == '.' {
                if cur_score + ns < scores[np][nd] {
                    q.push_back((np, nd));
                    scores[np][nd] = cur_score + ns;
                }
            }
        }
    }
    let bscore = *scores[end].iter().min().unwrap();
    for (i, p) in input.iter_mut().enumerate() {
        let bi = *scores[i].iter().min().unwrap();
        if bi > bscore {
            *p = '#';
        }
    }
    let mut best = vec![[false; 4]; input.len()];
    let mz = Maze {
        input: &input,
        scores: &scores,
        directions: &directions,
    };
    mz.dfs(&mut best, start, 0, end, 0);

    best.iter().filter(|x| x.iter().any(|y| *y)).count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 7036);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 9021);
    }
}
