use crate::util;

pub fn p2() {
    let contents = util::read_input();

    let split = contents.lines().map(|x| util::split_list(x)).collect::<Vec<_>>();

    let mut sum = 0;

    for row in split {
        if util::is_safe(&row) {
            sum += 1;
        } else {
            for (i, _) in row.iter().enumerate() {
                let mut row_copy = row.clone();
                row_copy.remove(i);

                if util::is_safe(&row_copy) {
                    sum += 1;
                    break;
                }
            }
        }
    }

    println!("Sum: {:#?}", sum);
}
