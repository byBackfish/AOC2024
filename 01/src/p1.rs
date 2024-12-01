use std::fs;

pub fn p1() {
    let contents = fs::read_to_string("input.txt").expect("wrong file path");

    let split = contents.lines().map(|x| split_list(x)).collect::<Vec<_>>();

    let mut left_side = split.iter().map(|x| x.0).collect::<Vec<_>>();
    let mut right_side = split.iter().map(|x| x.1).collect::<Vec<_>>();

    left_side.sort();
    right_side.sort();

    let mut difference = 0;

    for i in 0..left_side.len() {
        if left_side[i] != right_side[i] {
            difference += (left_side[i] - right_side[i]).abs();
        }
    }

    println!("The difference is: {}", difference);
}

fn split_list(input: &str) -> (i64, i64) {
    let split = input.split("   ");
    let vec = split.collect::<Vec<_>>();
    return (vec[0].parse().unwrap(), vec[1].parse().unwrap());
}
