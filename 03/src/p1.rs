use crate::util::{self, parse_instructions};

pub fn p1() {
    let contents = util::read_input();

    let calls = parse_instructions(contents);

    let result = calls.iter()
        .filter_map(|call| match call {
            util::Instruction::Mul(mul) => Some(mul),
            _ => None,
        })
        .fold(0, |acc, call| acc + call.a * call.b);

    println!("Result: {}", result);
}
