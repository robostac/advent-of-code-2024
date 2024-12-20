use std::{
    collections::{HashMap, VecDeque},
    usize,
};
fn do_move(p: usize, d: i64) -> usize {
    (p as i64 + d) as usize
}

fn make_scores(input: &[char], start: usize, line_length: i64) -> Vec<usize> {
    let directions = [1, line_length, -1, -line_length];
    let mut scores = vec![usize::MAX; input.len()];
    let mut q = VecDeque::new();
    q.push_back(start);
    scores[start] = 0;
    while let Some(p) = q.pop_front() {
        let cur_score = scores[p];
        for &d in directions.iter() {
            let np = do_move(p, d);
            if input[np] == '.' {
                if cur_score + 1 < scores[np] {
                    q.push_back(np);
                    scores[np] = cur_score + 1;
                }
            }
        }
    }
    scores
}

#[aoc(day20, part1)]
pub fn part1(input_struct: &str) -> usize {
    let mut input: Vec<char> = input_struct.chars().collect();
    let line_length = input.iter().position(|x| *x == '\n').unwrap() as i64 + 1;
    let start = input.iter().position(|x| *x == 'S').unwrap();
    let end = input.iter().position(|x| *x == 'E').unwrap();

    input[start] = '.';
    input[end] = '.';
    let from_start = make_scores(&input, start, line_length);
    let from_end = make_scores(&input, end, line_length);
    let best = from_start[end];
    let mut cheats = 0;
    for (p, c) in input.iter().enumerate() {
        if *c != '#' {
            continue;
        }
        for d in [1, line_length, -1, -line_length] {
            let prev = do_move(p, -d);
            let next = do_move(p, d);
            let ss = *from_start.get(prev).unwrap_or(&usize::MAX);
            let ee = *from_end.get(next).unwrap_or(&usize::MAX);

            if ss < usize::MAX && ee < usize::MAX {
                let new_time = ss + ee + 2;
                if best > new_time && best - new_time >= 100 {
                    cheats += 1;
                }
            }
        }
    }
    cheats
}

#[aoc(day20, part2)]
pub fn part2(input_struct: &str) -> i64 {
    let mut input: Vec<char> = input_struct.chars().collect();
    let line_length = input.iter().position(|x| *x == '\n').unwrap() as i64 + 1;
    let start = input.iter().position(|x| *x == 'S').unwrap();
    let end = input.iter().position(|x| *x == 'E').unwrap();

    input[start] = '.';
    input[end] = '.';
    let from_start = make_scores(&input, start, line_length);
    let from_end = make_scores(&input, end, line_length);
    let best = from_start[end];
    let mut cheats = 0;

    for (p, c) in input.iter().enumerate() {
        if *c != '.' {
            continue;
        }
        let ss = *from_start.get(p).unwrap_or(&usize::MAX);
        let x = p as i64 % line_length;
        let y = p as i64 / line_length;

        for nx in (-(x.min(20)))..=20 {
            if x + nx >= line_length {
                break;
            }
            let maxy = 20 - nx.abs();
            for ny in -(y.min(maxy))..=(maxy) {
                let dist = (nx.abs() + ny.abs()) as usize;
                let end = do_move(p, nx + ny * line_length);
                if end >= from_end.len() {
                    break;
                }
                let ee = *from_end.get(end).unwrap_or(&usize::MAX);

                if ss < usize::MAX && ee < usize::MAX {
                    let new_time = ss + ee + dist;
                    if best > new_time && best - new_time >= 100 {
                        cheats += 1;
                    }
                }
            }
        }
    }

    cheats
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    const TESTLIST: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    #[test]
    fn sample1() {
        assert_eq!(part1(&TESTLIST), 0);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&TESTLIST), 16);
    }
}
