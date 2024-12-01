fn main() {
    let input = include_str!("./input.txt");

    let split_cols: Vec<&str> = input.split("\n").collect();

    let split_rows: Vec<Vec<&str>> = split_cols
        .iter()
        .map(|&x| x.split("  ").collect())
        .collect();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for row in split_rows {
        match row[0].trim().parse::<i32>() {
            Ok(num) => left.push(num),
            Err(_) => continue,
        }

        match row[1].trim().parse::<i32>() {
            Ok(num) => right.push(num),
            Err(_) => continue,
        }
    }

    left.sort();
    right.sort();

    let mut distance: Vec<i32> = vec![];

    for (i, num) in left.iter().enumerate() {
        distance.push((num - right[i]).abs());
    }

    println!("{}", distance.iter().sum::<i32>())
}
