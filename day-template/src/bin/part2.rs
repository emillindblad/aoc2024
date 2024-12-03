fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("");
        assert_eq!(result, 0)
    }
}
