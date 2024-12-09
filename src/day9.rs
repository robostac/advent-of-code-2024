use std::collections::VecDeque;

fn quick_checksum(pos: usize, id: usize, sz: usize) -> usize {
    ((sz * (sz - 1)) / 2 + (sz * pos)) * id
}

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
        checksum += quick_checksum(cur_pos, fwd_id, v);
        cur_pos += v;
        fwd_id += 1;
        if let Some(mut empty_space) = cc.pop_front() {
            while empty_space > 0 {
                if cc.len() == 0 {
                    break;
                }
                if back_len == 0 {
                    back_len = cc.pop_back().unwrap();
                    back_id -= 1;
                    cc.pop_back();
                }
                let sz = empty_space.min(back_len);
                checksum += quick_checksum(cur_pos, back_id, sz);
                cur_pos += sz;
                back_len -= sz;
                empty_space -= sz;
            }
        }
    }
    checksum += quick_checksum(cur_pos, back_id, back_len);
    checksum
}

#[aoc(day9, part2)]
pub fn part2(input_struct: &str) -> usize {
    let mut cc = input_struct
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<VecDeque<_>>();
    let mut files = vec![Vec::new(); 10];
    for (i, x) in cc.iter().step_by(2).enumerate() {
        files[*x].push(i);
    }
    let mut fwd_id = 0;
    let mut cur_pos = 0;
    let mut checksum = 0;
    let mut used_ids = vec![false; cc.len() + 1 / 2];
    while let Some(v) = cc.pop_front() {
        if used_ids[fwd_id] == false {
            used_ids[fwd_id] = true;
            checksum += quick_checksum(cur_pos, fwd_id, v);
        }
        cur_pos += v;
        fwd_id += 1;
        if let Some(mut empty_space) = cc.pop_front() {
            while let Some(p) = (1..=empty_space)
                .filter(|x| files[*x].len() > 0)
                .max_by_key(|x| (*files[*x].last().unwrap_or(&0)))
            {
                let nid = files[p].pop().unwrap();
                let sz = p;
                if used_ids[nid] == false {
                    checksum += quick_checksum(cur_pos, nid, sz);
                    cur_pos += sz;
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
