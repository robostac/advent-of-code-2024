use std::{collections::VecDeque, usize};
fn do_move(p: usize, d: i64) -> usize {
    (p as i64 + d) as usize
}

fn make_scores(
    input: &[char],
    start: usize,
    line_length: i64,
    end: usize,
) -> (Vec<[usize; 4]>, usize) {
    let directions = [1, line_length, -1, -line_length];
    let mut scores = vec![[usize::MAX; 4]; input.len()];
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    scores[start][0] = 0;
    let mut bscore = usize::MAX;
    while let Some((p, d)) = q.pop_front() {
        let cur_score = scores[p][d];
        if cur_score > bscore {
            continue;
        }
        if p == end {
            bscore = bscore.min(cur_score);
        }
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
    (scores, bscore)
}

#[aoc(day16, part1)]
pub fn part1(input_struct: &str) -> usize {
    let mut input: Vec<char> = input_struct.chars().collect();
    let line_length = input.iter().position(|x| *x == '\n').unwrap() as i64 + 1;
    let start = input.iter().position(|x| *x == 'S').unwrap();
    let end = input.iter().position(|x| *x == 'E').unwrap();

    input[start] = '.';
    input[end] = '.';
    let (_, v) = make_scores(&input, start, line_length, end);
    v
}

struct Maze<'a> {
    scores: &'a [[usize; 4]],
    input: &'a [char],
    directions: &'a [i64],
}

impl Maze<'_> {
    fn dfs(&self, on_path: &mut [usize], pos: usize, d: usize, end: usize, cur: usize) -> bool {
        if self.scores[pos][d] != cur {
            return false;
        }
        if self.input[pos] == '#' {
            return false;
        }
        if (on_path[pos] & (1 << d)) > 0 {
            return true;
        }
        if pos == end {
            on_path[pos] |= 1 << d;
            return true;
        }
        for (np, nd, ns) in [
            (do_move(pos, -self.directions[d]), d, 1),
            (pos, (d + 1) % 4, 1000),
            (pos, (d + 3) % 4, 1000),
        ] {
            if self.dfs(on_path, np, nd, end, cur - ns) {
                on_path[pos] |= 1 << d;
            }
        }
        return (on_path[pos] & (1 << d)) > 0;
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
    let (scores, bscore) = make_scores(&input, start, line_length, end);

    let mut best = vec![0; input.len()];
    let mz = Maze {
        input: &input,
        scores: &scores,
        directions: &directions,
    };
    for i in 0..4 {
        mz.dfs(&mut best, end, i, start, bscore);
    }

    best.iter().filter(|x| **x > 0).count()
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
        assert_eq!(part2(TESTLIST), 45);
    }
}
