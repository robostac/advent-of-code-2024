#[aoc(day8, part1)]
pub fn part1(input_struct: &str) -> usize {
    let line_length = input_struct.chars().position(|x| x == '\n').unwrap() as i64 + 1;
    let mut locations = vec![Vec::new(); 128];
    let mut positions = vec![false; input_struct.len()];
    for (p, c) in input_struct.chars().enumerate() {
        let cc = c as usize;
        if c != '\n' && c != '.' {
            locations[cc].push(p);
        }
    }
    for points in locations {
        for (i, &p) in points.iter().enumerate() {
            let px = p as i64 % line_length;
            for &pp in points.iter().skip(i + 1) {
                let ppx = pp as i64 % line_length;
                for (yp, zp, yx, zx) in [
                    (pp as i64, p as i64, ppx, px),
                    (p as i64, pp as i64, px, ppx),
                ] {
                    let dx = (yx) - (zx);
                    let nx = yx + dx;
                    if nx < 0 || nx >= (line_length - 1) {
                        continue;
                    }
                    let npos = (yp - zp) + yp;
                    if let Some(e) = positions.get_mut((npos) as usize) {
                        *e = true;
                    }
                }
            }
        }
    }
    positions.iter().filter(|z| **z).count()
}

#[aoc(day8, part2)]
pub fn part2(input_struct: &str) -> usize {
    let line_length = input_struct.chars().position(|x| x == '\n').unwrap() as i64 + 1;
    let mut locations = vec![Vec::new(); 128];
    let mut positions = vec![false; input_struct.len()];
    for (p, c) in input_struct.chars().enumerate() {
        let cc = c as usize;
        if c != '\n' && c != '.' {
            locations[cc].push(p);
        }
    }
    for points in locations {
        for (i, &p) in points.iter().enumerate() {
            let px = p as i64 % line_length;
            for &pp in points.iter().skip(i + 1) {
                let ppx = pp as i64 % line_length;
                let dd = (pp as i64) - (p as i64);
                let ddx = ppx - px;
                for (mut y, mut yx, d, dx) in [(p as i64, px, dd, ddx), (p as i64, px, -dd, -ddx)] {
                    loop {
                        if yx < 0 || yx >= (line_length - 1) {
                            break;
                        }
                        if let Some(e) = positions.get_mut((y) as usize) {
                            *e = true;
                            y += d;
                            yx += dx;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    positions.iter().filter(|z| **z).count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 14);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 34);
    }
}
