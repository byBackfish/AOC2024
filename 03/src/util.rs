use regex::Regex;

pub struct Mul {
    pub a: i32,
    pub b: i32,
}

pub enum Instruction {
    Mul(Mul),
    Do,
    Dont,
}

pub fn read_input() -> String {
    std::fs::read_to_string("input.txt").expect("wrong file path")
}

pub fn parse_instructions(input: String) -> Vec<Instruction> {
    let regex = Regex::new(r"(?m)(mul|do|don't)(?:\((\d{1,3}),(\d+)\)|\(\))").unwrap();
    let mut instructions = Vec::new();

    for capture in regex.captures_iter(&input) {
        let instruction = match &capture[1] {
            "do" => Instruction::Do,
            "don't" => Instruction::Dont,
            _ => {
                let a = capture[2].parse().unwrap();
                let b = capture[3].parse().unwrap();

                Instruction::Mul(Mul { a, b })
            }
        };

        instructions.push(instruction);
    }

    instructions
}
