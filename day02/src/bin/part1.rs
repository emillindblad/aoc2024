fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> i32 {
    let mut safe_reports = 0;

    'outer: for line in input.lines() {
        let split_line: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if split_line[0] == split_line[1] {
            continue;
        }

        let increasing = split_line[0] < split_line[1];
        for pair in split_line.windows(2) {
            if !is_safe(pair[0], pair[1], increasing) {
                continue 'outer;
            }
        }
        safe_reports += 1;
    }
    safe_reports
}

fn is_safe(left: i32, right: i32, increasing: bool) -> bool {
    if increasing {
        if left >= right || right - left > 3 {
            return false;
        }
    } else if left <= right || left - right > 3 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9",
        );
        assert_eq!(result, 2)
    }
}
