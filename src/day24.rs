use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule<'a> {
    XOR(&'a str, &'a str),
    AND(&'a str, &'a str),
    OR(&'a str, &'a str),
}

fn search(v: &str, rules: &HashMap<&str, Rule>, values: &mut HashMap<String, u8>) -> u8 {
    if let Some(a) = values.get(v) {
        return *a;
    }

    let nv = match rules[v] {
        Rule::XOR(a, b) => search(a, rules, values) ^ search(b, rules, values),
        Rule::AND(a, b) => search(a, rules, values) & search(b, rules, values),
        Rule::OR(a, b) => search(a, rules, values) | search(b, rules, values),
    };

    values.insert(v.to_owned(), nv);
    nv
}

#[aoc(day24, part1)]
pub fn part1(input_struct: &str) -> usize {
    let mut values = HashMap::new();
    let (a, b) = input_struct.split_once("\n\n").unwrap();
    for l in a.lines() {
        let id = &l[..3];
        let val = l[5..=5].parse::<u8>().unwrap();
        values.insert(id.to_owned(), val);
    }
    let mut rules = HashMap::new();
    for l in b.lines() {
        let z = l.split_ascii_whitespace().collect::<Vec<_>>();
        let r = match z[1] {
            "XOR" => Rule::XOR(z[0], z[2]),
            "AND" => Rule::AND(z[0], z[2]),
            _ => Rule::OR(z[0], z[2]),
        };
        rules.insert(z[4], r);
    }
    let mut ans = 0;
    for b in 0.. {
        let s = format!("z{:02}", b);
        if rules.contains_key(&*s) {
            let v = search(&s, &rules, &mut values) as usize;

            ans |= v << b;
        } else {
            break;
        }
    }
    ans
}

#[aoc(day24, part2)]
pub fn part2(input_struct: &str) -> String {
    let mut values = HashMap::new();
    let (a, b) = input_struct.split_once("\n\n").unwrap();
    for l in a.lines() {
        let id = &l[..3];
        let val = l[5..=5].parse::<u8>().unwrap();
        values.insert(id.to_owned(), val);
    }
    let mut rules = HashMap::new();
    for l in b.lines() {
        let z = l.split_ascii_whitespace().collect::<Vec<_>>();
        let r = match z[1] {
            "XOR" => Rule::XOR(z[0], z[2]),
            "AND" => Rule::AND(z[0], z[2]),
            _ => Rule::OR(z[0], z[2]),
        };
        rules.insert(z[4], r);
    }

    //solved by looking at output - each stage needs 2xAND + 2xXOR + OR
    let mut swaps = ["hcc", "z11", "z35", "fdw", "z05", "bpf", "hqc", "qcw"];
    for i in swaps.chunks(2) {
        let a = i[0];
        let b = i[1];
        let r1 = rules[a].clone();
        let r2 = rules[b].clone();
        rules.insert(a, r2);
        rules.insert(b, r1);
    }

    for b in 0.. {
        let s = format!("z{:02}", b);
        if rules.contains_key(&*s) {
            let v = search(&s, &rules, &mut values) as usize;
        } else {
            break;
        }
    }
    swaps.sort();
    swaps.join(",")
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    const TESTLIST: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    #[test]
    fn sample1() {
        assert_eq!(part1(&TESTLIST), 2024);
    }
}
