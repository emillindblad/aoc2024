fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> i32 {
    let mut input_mat: Vec<Vec<char>> = vec![];

    let mut wc = 0;
    for line in input.lines() {
        let line_chars: Vec<char> = line.chars().collect();
        input_mat.push(line_chars);

        // println!("{line}");
        let xmas_matches = line.matches("XMAS").count();
        let samx_matches = line.matches("SAMX").count();
        wc += xmas_matches;
        wc += samx_matches;
        // println!("xmas: {xmas_matches}, smax {samx_matches}");
    }

    for (i, row) in input_mat.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if char == &'X' {
                if input_mat.get(i + 1).and_then(|r| r.get(j)) == Some(&'M')
                    && input_mat.get(i + 2).and_then(|r| r.get(j)) == Some(&'A')
                    && input_mat.get(i + 3).and_then(|r| r.get(j)) == Some(&'S')
                {
                    wc += 1;
                }
                if input_mat.get(i + 1).and_then(|r| r.get(j + 1)) == Some(&'M')
                    && input_mat.get(i + 2).and_then(|r| r.get(j + 2)) == Some(&'A')
                    && input_mat.get(i + 3).and_then(|r| r.get(j + 3)) == Some(&'S')
                {
                    wc += 1;
                }
                if j >= 3
                    && input_mat.get(i + 1).and_then(|r| r.get(j - 1)) == Some(&'M')
                    && input_mat.get(i + 2).and_then(|r| r.get(j - 2)) == Some(&'A')
                    && input_mat.get(i + 3).and_then(|r| r.get(j - 3)) == Some(&'S')
                {
                    wc += 1;
                }
            }

            if char == &'S' {
                if input_mat.get(i + 1).and_then(|r| r.get(j)) == Some(&'A')
                    && input_mat.get(i + 2).and_then(|r| r.get(j)) == Some(&'M')
                    && input_mat.get(i + 3).and_then(|r| r.get(j)) == Some(&'X')
                {
                    wc += 1;
                }
                if input_mat.get(i + 1).and_then(|r| r.get(j + 1)) == Some(&'A')
                    && input_mat.get(i + 2).and_then(|r| r.get(j + 2)) == Some(&'M')
                    && input_mat.get(i + 3).and_then(|r| r.get(j + 3)) == Some(&'X')
                {
                    wc += 1;
                }
                if j >= 3
                    && input_mat.get(i + 1).and_then(|r| r.get(j - 1)) == Some(&'A')
                    && input_mat.get(i + 2).and_then(|r| r.get(j - 2)) == Some(&'M')
                    && input_mat.get(i + 3).and_then(|r| r.get(j - 3)) == Some(&'X')
                {
                    wc += 1;
                }
            }
        }
    }

    // Find X or S
    // Find pos in row, check if m[pos][+1,+2,+3] == XMAS or SAMX

    // println!("{:?},{:?}", 1, mat_col.expect("End of row"));

    wc.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX",
        );
        assert_eq!(result, 18)
    }
}
