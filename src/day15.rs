#[aoc(day15, part1)]
pub fn part1(input_struct: &str) -> usize {
    let mut grid = Vec::new();
    let mut moves = Vec::new();
    let mut first = true;
    for l in input_struct.lines() {
        if l.len() == 0 {
            first = false;
        } else if first {
            grid.extend(l.chars());
        } else {
            moves.extend(l.chars());
        }
    }
    let line_length = input_struct.chars().position(|x| x == '\n').unwrap() as i64;
    let llu: usize = line_length as usize;
    let mut fish_pos = grid.iter().position(|x| *x == '@').unwrap();
    grid[fish_pos] = '.';

    for m in moves {
        let d = match m {
            '>' => 1,
            '<' => -1,
            '^' => -line_length,
            'v' => line_length,
            _ => panic!(),
        };
        let next_fish_pos = (fish_pos as i64 + d) as usize;
        fish_pos = match grid[next_fish_pos] {
            '#' => fish_pos,
            'O' => {
                let mut box_pos = next_fish_pos;
                while grid[box_pos] == 'O' {
                    box_pos = (box_pos as i64 + d) as usize
                }
                if grid[box_pos] == '.' {
                    grid[next_fish_pos] = '.';
                    grid[box_pos] = 'O';
                    next_fish_pos
                } else {
                    fish_pos
                }
            }
            _ => next_fish_pos,
        };
    }

    grid.iter()
        .enumerate()
        .map(|(p, v)| {
            if *v == 'O' {
                (100 * (p / (llu))) + (p % (llu))
            } else {
                0
            }
        })
        .sum()
}

enum Cell {
    Wall,
    Space,
    Box(usize),
}

fn do_move(p: usize, d: i64) -> usize {
    (p as i64 + d) as usize
}

#[aoc(day15, part2)]
pub fn part2(input_struct: &str) -> usize {
    let mut boxes = Vec::new();
    let mut grid = Vec::new();
    let mut moves = Vec::new();
    let mut first = true;
    let mut fish_pos = 0;
    let line_length = input_struct.chars().position(|x| x == '\n').unwrap() as i64 * 2;
    let llu: usize = line_length as usize;
    for l in input_struct.lines() {
        if l.len() == 0 {
            first = false;
        } else if first {
            for c in l.chars() {
                match c {
                    '@' => {
                        fish_pos = grid.len();
                        grid.push(Cell::Space);
                        grid.push(Cell::Space);
                    }
                    '.' => {
                        grid.push(Cell::Space);
                        grid.push(Cell::Space);
                    }
                    'O' => {
                        let pos = grid.len();
                        grid.push(Cell::Box(boxes.len()));
                        grid.push(Cell::Box(boxes.len()));
                        boxes.push(pos);
                    }
                    _ => {
                        grid.push(Cell::Wall);
                        grid.push(Cell::Wall);
                    }
                }
            }
        } else {
            moves.extend(l.chars());
        }
    }

    let mut pushed_id = vec![usize::MAX; boxes.len()];
    let mut pushed_queue = vec![0; boxes.len()];
    for (i, &m) in moves.iter().enumerate() {
        let d = match m {
            '>' => 1,
            '<' => -1,
            '^' => -line_length,
            'v' => line_length,
            _ => panic!(),
        };
        let next_fish_pos = (fish_pos as i64 + d) as usize;
        fish_pos = match grid[next_fish_pos] {
            Cell::Wall => fish_pos,
            Cell::Box(b) => {
                let mut idx = 1;
                pushed_queue[0] = b;
                pushed_id[b] = i;
                let mut sp = 0;
                let mut do_push = true;
                while sp < idx && do_push {
                    let bid = pushed_queue[sp];
                    sp += 1;
                    let bpos = boxes[bid];
                    for np in [do_move(bpos, d), do_move(bpos, d + 1)] {
                        match grid[np] {
                            Cell::Space => (),
                            Cell::Box(box_id) => {
                                if pushed_id[box_id] == i {
                                    continue;
                                }
                                pushed_id[box_id] = i;
                                pushed_queue[idx] = box_id;
                                idx += 1;
                            }
                            _ => do_push = false,
                        }
                    }
                }
                if do_push {
                    for i in (0..idx).rev() {
                        let obp = boxes[pushed_queue[i]];
                        let nbp = do_move(obp, d);
                        if d == -1 {
                            grid.swap(nbp, obp);
                            grid.swap(nbp + 1, obp + 1);
                        } else {
                            grid.swap(nbp + 1, obp + 1);
                            grid.swap(nbp, obp);
                        }
                        boxes[pushed_queue[i]] = nbp;
                    }
                    next_fish_pos
                } else {
                    fish_pos
                }
            }
            _ => next_fish_pos,
        };
    }
    boxes
        .iter()
        .map(|p| (100 * (p / (llu))) + (p % (llu)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 10092);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 9021);
    }
}
