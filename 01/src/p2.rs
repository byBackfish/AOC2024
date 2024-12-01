use std::fs;

pub fn p2() {
    let contents = fs::read_to_string("input.txt").expect("wrong file path");

    let split = contents.lines().map(|x| split_list(x)).collect::<Vec<_>>();

    let left_side = split.iter().map(|x| x.0).collect::<Vec<_>>();
    let right_side = split.iter().map(|x| x.1).collect::<Vec<_>>();

    let mut similarity = 0;

    for left_i in 0..left_side.len() {
        for right_i in 0..right_side.len() {
            if left_side[left_i] == right_side[right_i] {
                similarity += left_side[left_i];
            }
        }
    }

    println!("The score is: {}", similarity);
}

fn split_list(input: &str) -> (i64, i64) {
    let split = input.split("   ");
    let vec = split.collect::<Vec<_>>();
    return (vec[0].parse().unwrap(), vec[1].parse().unwrap());
}
