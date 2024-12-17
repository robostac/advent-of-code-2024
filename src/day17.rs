fn parse_input(x: &str) -> Vec<i64> {
    x.split(|x| x < '0' || x > '9')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect()
}

fn combo(v: i64, reg: &[i64]) -> i64 {
    match v {
        4 => reg[0],
        5 => reg[1],
        6 => reg[2],
        7 => panic!(),
        p => p,
    }
}

fn run(registers: &mut [i64], inst: &[i64]) -> Vec<i64> {
    let mut output = Vec::new();
    let mut pc = 0;
    while pc < inst.len() {
        let op = inst[pc];
        let val = inst[pc + 1];
        pc += 2;
        match op {
            0 => registers[0] = registers[0] / 2i64.pow(combo(val, &registers) as u32),
            1 => registers[1] = registers[1] ^ val,
            2 => registers[1] = combo(val, &registers) % 8,
            3 => {
                if registers[0] != 0 {
                    pc = val as usize;
                }
            }
            4 => registers[1] = registers[1] ^ registers[2],
            5 => output.push(combo(val, &registers) % 8),
            6 => registers[1] = registers[0] / 2i64.pow(combo(val, &registers) as u32),
            7 => registers[2] = registers[0] / 2i64.pow(combo(val, &registers) as u32),
            _ => panic!(),
        }
    }
    output
}

#[aoc(day17, part1)]
pub fn part1(input_struct: &str) -> String {
    let inp = parse_input(input_struct);

    let mut registers = [inp[0], inp[1], inp[2]];
    let output = run(&mut registers, &inp[3..]);
    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn dfs2(tgt: &[i64], sval: i64, bbb: usize, inst: &[i64]) -> Option<i64> {
    for i in 0..8 {
        let sv = sval | i << bbb;
        let mut registers = [sv, 0, 0];
        let output = run(&mut registers, inst);

        if output.len() >= tgt.len() {
            if output[tgt.len() - 1] == tgt[tgt.len() - 1] {
                if tgt.len() == 1 {
                    return Some(sv);
                }
                let ans = dfs2(&tgt[..tgt.len() - 1], sv, bbb - 3, inst);
                if ans.is_some() {
                    return ans;
                }
            }
        }
    }

    None
}

#[aoc(day17, part2)]
pub fn part2(input_struct: &str) -> i64 {
    let inp = parse_input(input_struct);

    let tgt = &inp[3..];

    let ll = 8usize.pow(tgt.len() as u32 - 1).trailing_zeros();
    dfs2(tgt, 0, ll as usize, tgt).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TESTLIST: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    #[test]
    fn sample1() {
        assert_eq!(part1(TESTLIST), "4,6,3,5,6,3,5,2,1,0");
    }

    const TEST2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    #[test]
    fn sample2() {
        assert_eq!(part2(TEST2), 117440);
    }
}
