use crate::util::{self, parse_instructions, Instruction};

pub fn p2() {
    let contents = util::read_input();

    let instructions = parse_instructions(contents);

    let mut enabled = true;
    let mut result = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Mul(mul) => if enabled {
                result += mul.a * mul.b;
            },
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        }
    }

    println!("Result: {}", result);
}
