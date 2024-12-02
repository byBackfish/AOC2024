pub fn read_input() -> String {
    std::fs::read_to_string("input.txt").expect("wrong file path")
}

pub fn split_list(input: &str) -> Vec<i32> {
    let split = input.split(" ");
    let vec = split.collect::<Vec<_>>();
    let parsed = vec.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    parsed
}

pub fn is_safe(row: &Vec<i32>) -> bool {
    let mut current = row.get(0).unwrap();

    for next in row.iter().skip(1) {
        let difference = next - current;

        if (difference > 0 && row.get(0) > row.get(1)) || (difference < 0 && row.get(0) < row.get(1)) {
            return false;
        }

        if difference.abs() > 3 || difference.abs() < 1 {
            return false;
        }

        current = next;
    }

    true
}
