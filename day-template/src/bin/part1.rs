fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("");
        assert_eq!(result, 0)
    }
}
