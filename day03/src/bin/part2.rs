use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut instruction_sum = 0;

    let mut do_instr = true;
    for cap in re.captures_iter(input) {
        match &cap[0] {
            "do()" => {
                do_instr = true;
                continue;
            }
            "don't()" => {
                do_instr = false;
                continue;
            }
            _ => {}
        }

        if do_instr {
            let left: i32 = cap[1].parse().unwrap_or(0);
            let right: i32 = cap[2].parse().unwrap_or(0);
            instruction_sum += left * right;
        }
    }

    instruction_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result =
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48)
    }
}
