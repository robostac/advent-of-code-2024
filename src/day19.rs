#[derive(Debug, Clone, Default)]
struct TNode {
    next: [Option<usize>; 26],
    terminal: bool,
}

impl TNode {
    fn new() -> Self {
        Self {
            next: Default::default(),
            terminal: false,
        }
    }
}

fn idx(c: char) -> usize {
    c as usize - 'a' as usize
}

fn possible(x: &str, node: usize, nodes: &[TNode], memo: &mut [usize], iter: usize) -> bool {
    let n = &nodes[node];
    if node == 0 {
        if memo[x.len()] == iter {
            return false;
        }
        memo[x.len()] = iter;
    }

    if n.terminal {
        if x.len() == 0 {
            return true;
        }
        if possible(x, 0, nodes, memo, iter) {
            return true;
        }
    }
    if x.len() == 0 {
        return false;
    }
    let c = x.chars().next().unwrap();
    let nx = &x[1..];
    if let Some(p) = nodes[node].next[idx(c)] {
        return possible(nx, p, nodes, memo, iter);
    }
    false
}

fn make_trie(a: &str) -> Vec<TNode> {
    let mut nodes = Vec::new();

    nodes.push(TNode::new());
    for x in a.split(',') {
        let mut node = 0;
        for c in x.trim().chars() {
            let idx = idx(c);
            if let Some(p) = nodes[node].next[idx] {
                node = p;
            } else {
                nodes[node].next[idx] = Some(nodes.len());
                node = nodes.len();
                nodes.push(TNode::new());
            }
        }
        nodes[node].terminal = true;
    }
    nodes
}

#[aoc(day19, part1)]
pub fn part1(input_struct: &str) -> i64 {
    let (a, b) = input_struct.split_once("\n\n").unwrap();
    let nodes = make_trie(a);
    let mut count = 0;
    let mut possible_memo = [0; 2000];
    for (i, l) in b.lines().enumerate() {
        let i = i + 1;
        if possible(l, 0, &nodes, &mut possible_memo, i) {
            count += 1;
        }
    }

    count
}

fn count_ways(
    x: &str,
    node: usize,
    nodes: &[TNode],
    memo: &mut [(usize, i64)],
    iter: usize,
) -> i64 {
    let n = &nodes[node];
    if node == 0 {
        if memo[x.len()].0 == iter {
            return memo[x.len()].1;
        }
    }
    let mut ways = 0;
    if n.terminal {
        if x.len() == 0 {
            return 1;
        }
        ways += count_ways(x, 0, nodes, memo, iter);
    }
    if x.len() == 0 {
        return 0;
    }
    let c = x.chars().next().unwrap();
    let nx = &x[1..];

    if let Some(p) = nodes[node].next[idx(c)] {
        ways += count_ways(nx, p, nodes, memo, iter);
    }
    if node == 0 {
        memo[x.len()] = (iter, ways);
    }
    ways
}

#[aoc(day19, part2)]
pub fn part2(input_struct: &str) -> i64 {
    let (a, b) = input_struct.split_once("\n\n").unwrap();
    let nodes = make_trie(a);
    let mut possible_memo = [(0, 0); 2000];
    let mut count = 0;
    for (i, l) in b.lines().enumerate() {
        let iter = i + 1;
        count += count_ways(l, 0, &nodes, &mut possible_memo, iter);
    }
    count
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    const TESTLIST: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    #[test]
    fn sample1() {
        assert_eq!(part1(&TESTLIST), 6);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&TESTLIST), 16);
    }
}
