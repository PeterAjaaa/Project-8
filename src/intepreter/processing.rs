use std::io::{self, Read};

pub fn interpret(mut data_tape: Vec<u8>, instructions_input: Vec<char>) {
    // Refers to the location or the position of the current instruction in the data tapes
    let mut pointer: usize = 0;

    for instruction in instructions_input {
        match instruction {
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '+' => data_tape[pointer] += 1,
            '-' => data_tape[pointer] -= 1,
            '.' => print!("{}", data_tape[pointer] as char),
            ',' => {
                let mut input = [0u8; 1];
                if io::stdin().read_exact(&mut input).is_ok() {
                    data_tape[pointer] = input[0];
                }
            }
            _ => (),
        }
    }
}
