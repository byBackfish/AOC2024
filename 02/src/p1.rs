use crate::util;

pub fn p1() {
    let contents = util::read_input();

    let split = contents.lines().map(|x| util::split_list(x)).collect::<Vec<_>>();

    let mut sum = 0;

    for row in split {
        let is_safe = util::is_safe(&row);

        if is_safe {
            sum += 1;
        }
    }

    println!("Sum: {:#?}", sum);
}
