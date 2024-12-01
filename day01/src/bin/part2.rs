use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let split_cols: Vec<&str> = input.split("\n").collect();

    let split_rows: Vec<Vec<&str>> = split_cols
        .iter()
        .map(|&x| x.split("  ").collect())
        .collect();

    let mut left = HashMap::new();
    let mut right = HashMap::new();

    for row in split_rows {
        match row[0].trim().parse::<i32>() {
            Ok(num) => *left.entry(num).or_insert(0) += 1,
            Err(_) => continue,
        }

        match row[1].trim().parse::<i32>() {
            Ok(num) => *right.entry(num).or_insert(0) += 1,
            Err(_) => continue,
        }
    }

    let mut sim_score: Vec<i32> = vec![];

    for (key, _) in &left {
        sim_score.push(key * right.get(key).unwrap_or(&0));
    }

    println!("{}", sim_score.iter().sum::<i32>())
}
