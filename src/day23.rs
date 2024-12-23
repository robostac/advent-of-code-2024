fn val(s: &str) -> usize {
    (s.chars().nth(0).unwrap() as usize - 'a' as usize) * 26
        + (s.chars().nth(1).unwrap() as usize - 'a' as usize)
}

const MAX_IDX: usize = 26 * 26;

#[aoc(day23, part1)]
pub fn part1(input_struct: &str) -> i64 {
    let mut links = vec![Vec::new(); MAX_IDX];
    let mut ll = [false; MAX_IDX * MAX_IDX];
    for l in input_struct.lines() {
        let (a, b) = l.split_once('-').unwrap();
        let va = val(&a);
        let vb = val(&b);
        links[va].push(vb);
        ll[va * MAX_IDX + vb] = true;
        links[vb].push(va);
        ll[vb * MAX_IDX + va] = true;
    }
    let tval = 't' as usize - 'a' as usize;
    let start = tval * 26;
    let mut total = 0;
    // let mut hs = HashSet::new();

    for z in 0..26 {
        let p1 = start + z;
        for (i, &p2) in links[p1].iter().enumerate() {
            if p2 / 26 == tval && p2 < p1 {
                continue;
            }
            for &p3 in links[p1].iter().skip(i + 1) {
                if p3 / 26 == tval && p3 < p1 {
                    continue;
                }
                if ll[p3 * MAX_IDX + p2] {
                    total += 1;
                }
            }
        }
    }
    total
}

fn map_to_string(x: usize) -> String {
    let mut s = String::new();
    s.push((x / 26 + 'a' as usize) as u8 as char);
    s.push((x % 26 + 'a' as usize) as u8 as char);
    s
}

#[aoc(day23, part2)]
pub fn part2(input_struct: &str) -> String {
    let mut links = vec![Vec::new(); MAX_IDX];
    let mut ll = [false; MAX_IDX * MAX_IDX];
    for l in input_struct.lines() {
        let (a, b) = l.split_once('-').unwrap();
        let va = val(&a);
        let vb = val(&b);
        if va < vb {
            links[va].push(vb);
        } else {
            links[vb].push(va);
        }
        ll[va * MAX_IDX + vb] = true;
        ll[vb * MAX_IDX + va] = true;
    }
    let mut biggest = Vec::new();
    for (i, ilink) in links.iter().enumerate() {
        if ilink.len() < biggest.len() {
            continue;
        }
        let mut queue = Vec::new();
        queue.push((i, 0, 1));

        for l in ilink.iter() {
            let longest = queue.len();
            for j in 0..longest {
                let mut idx = j;
                let mut linked = true;
                while queue[idx].0 != i {
                    if ll[queue[idx].0 * MAX_IDX + *l] == false {
                        linked = false;
                        break;
                    }
                    idx = queue[idx].1;
                }
                if linked {
                    queue.push((*l, j, queue[j].2 + 1));
                }
            }
        }
        let mut test = Vec::new();
        let mut node = queue.iter().max_by_key(|x| x.2).unwrap();

        loop {
            test.push(node.0);
            if node.0 == i {
                break;
            }
            node = &queue[node.1];
        }
        if test.len() > biggest.len() {
            biggest = test;
        }
    }
    let mut biggest = biggest
        .into_iter()
        .map(|x| map_to_string(x))
        .collect::<Vec<_>>();
    biggest.sort();
    biggest.join(",")
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    const TESTLIST: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    #[test]
    fn sample1() {
        assert_eq!(part1(&TESTLIST), 7);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&TESTLIST), "co,de,ka,ta");
    }
}
