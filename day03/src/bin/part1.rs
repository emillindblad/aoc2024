use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut instruction_sum = 0;

    for cap in re.captures_iter(input) {
        let left: i32 = cap[1].parse().unwrap();
        let right: i32 = cap[2].parse().unwrap();
        instruction_sum += left * right;
    }

    instruction_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result =
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161)
    }
}
