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

    let mut mat_row = input_mat.iter();
    let mat_col = mat_row.next();
    // Find X or S
    // Find pos in row, check if m[pos][+1,+2,+3] == XMAS or SAMX

    println!("{:?},{:?}", 1, mat_col.expect("End of row"));

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
