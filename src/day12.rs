use std::collections::VecDeque;

#[aoc(day12, part1)]
pub fn part1(input_struct: &str) -> u64 {
    let line_length = input_struct.chars().position(|x| x == '\n').unwrap() as i64 + 1;
    let input = input_struct.chars().collect::<Vec<_>>();
    let mut visited = vec![false; input.len()];
    let mut vdq = VecDeque::new();
    let mut ans = 0;
    for (i, &c) in input.iter().enumerate() {
        if c == '\n' || visited[i] {
            continue;
        }
        vdq.push_back(i);
        let mut area = 0;
        let mut boundary = 0;
        visited[i] = true;
        while let Some(pos) = vdq.pop_front() {
            area += 1;
            for d in [1, -1, line_length, -line_length] {
                let np = (pos as i64 + d) as usize;
                let next = *input.get(np).unwrap_or(&'\n');
                if next != c {
                    boundary += 1;
                } else if !visited[np] {
                    visited[np] = true;
                    vdq.push_back(np);
                }
            }
        }

        ans += area * boundary;
    }
    ans
}

#[aoc(day12, part2)]
pub fn part2(input_struct: &str) -> u64 {
    let line_length = input_struct.chars().position(|x| x == '\n').unwrap() as i64 + 1;
    let input = input_struct.chars().collect::<Vec<_>>();
    let mut visited = vec![false; input.len()];
    let mut vdq = VecDeque::new();
    let mut ans = 0;
    for (i, &c) in input.iter().enumerate() {
        if c == '\n' || visited[i] {
            continue;
        }
        vdq.push_back(i);
        let mut area = 0;
        let mut boundary = 0;
        visited[i] = true;
        while let Some(pos) = vdq.pop_front() {
            area += 1;
            for (i, &d) in [1, -1, line_length, -line_length].iter().enumerate() {
                let np = (pos as i64 + d) as usize;
                let next = *input.get(np).unwrap_or(&'\n');
                if next != c {
                    let other_pos;
                    let second_other_pos;
                    if i < 2 {
                        other_pos = *input
                            .get((pos as i64 - line_length) as usize)
                            .unwrap_or(&'\n');
                        second_other_pos = *input
                            .get((np as i64 - line_length) as usize)
                            .unwrap_or(&'\n');
                    } else {
                        other_pos = *input.get((pos as i64 - 1) as usize).unwrap_or(&'\n');
                        second_other_pos = *input.get((np as i64 - 1) as usize).unwrap_or(&'\n');
                    }

                    if other_pos != c || (other_pos == c && second_other_pos == c) {
                        boundary += 1;
                    }
                } else if !visited[np] {
                    visited[np] = true;
                    vdq.push_back(np);
                }
            }
        }
        ans += area * boundary;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    const SIMPLE_TEST: &str = "AAAA
BBCD
BBCC
EEEC";
    #[test]
    fn simple1() {
        assert_eq!(part1(SIMPLE_TEST), 140);
    }
    #[test]
    fn simple2() {
        assert_eq!(part2(SIMPLE_TEST), 80);
    }

    const TESTLIST: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 1930);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 1206);
    }
}
