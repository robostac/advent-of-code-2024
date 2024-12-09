use std::collections::VecDeque;

#[aoc(day9, part1)]
pub fn part1(input_struct: &str) -> usize {
    let mut cc = input_struct
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<VecDeque<_>>();
    let mut back_id = cc.len() / 2 + 1;
    let mut back_len = 0;

    let mut cur_pos = 0;
    let mut fwd_id = 0;
    let mut checksum = 0;
    while let Some(v) = cc.pop_front() {
        for _ in 0..v {
            checksum += cur_pos * fwd_id;
            cur_pos += 1;
        }
        fwd_id += 1;
        if let Some(empty_space) = cc.pop_front() {
            for _ in 0..empty_space {
                if cc.len() == 0 {
                    break;
                }
                if back_len == 0 {
                    back_len = cc.pop_back().unwrap();
                    back_id -= 1;
                    cc.pop_back();
                }
                checksum += cur_pos * back_id;
                cur_pos += 1;
                back_len -= 1;
            }
        }
    }
    for _ in 0..back_len {
        checksum += cur_pos * back_id;
        cur_pos += 1;
        back_len -= 1;
    }
    // for c in inpu/t_struct.chars() {}
    checksum
}

#[aoc(day9, part2)]
pub fn part2(input_struct: &str) -> usize {
    let mut cc = input_struct
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<VecDeque<_>>();
    let mut files = vec![Vec::new(); 10];
    let mut id = 0;
    let mut pos = 0;
    for (i, x) in cc.iter().enumerate() {
        if i % 2 == 0 {
            files[*x].push(id);
            id += 1;
        }
        pos += x
    }
    let mut fwd_id = 0;
    let mut cur_pos = 0;
    let mut checksum = 0;
    let mut used_ids = vec![false; cc.len()];
    while let Some(v) = cc.pop_front() {
        if used_ids[fwd_id] == false {
            used_ids[fwd_id] = true;
            for _ in 0..v {
                checksum += cur_pos * fwd_id;
                cur_pos += 1;
            }
        } else {
            cur_pos += v;
        }
        fwd_id += 1;
        if let Some(mut empty_space) = cc.pop_front() {
            while let Some(p) = (1..=empty_space)
                .filter(|x| files[*x].len() > 0)
                .max_by_key(|x| (*files[*x].iter().last().unwrap_or(&0)))
            {
                let nid = files[p].pop().unwrap();
                let sz = p;
                if used_ids[nid] == false {
                    for _ in 0..sz {
                        checksum += cur_pos * nid;
                        cur_pos += 1;
                    }
                    empty_space -= sz;
                    used_ids[nid] = true;
                }
            }
            cur_pos += empty_space;
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "2333133121414131402";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), 1928);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TESTLIST), 2858);
    }
}
