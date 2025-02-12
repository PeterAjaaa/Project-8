/// This file contains deprecated functions, that is being kept for history purposes.
use std::{
    collections::HashMap,
    io::{self, Read},
};

#[allow(dead_code)]
fn process(data_tape: &mut Vec<u8>, instructions_input: Vec<char>) {
    // Refers to the location or the position of the current instruction in the data tapes
    let mut mem_cell_number: usize = 0;
    // Refers to the instruction individual position.
    let mut instruction_number: usize = 0;
    let mut loop_pair: HashMap<usize, usize> = HashMap::new();
    let mut stack: Vec<usize> = Vec::new();

    for (pos, instruction) in instructions_input.iter().enumerate() {
        match instruction {
            '[' => stack.push(pos),
            ']' => {
                if let Some(start) = stack.pop() {
                    loop_pair.insert(start, pos);
                    loop_pair.insert(pos, start);
                }
            }
            _ => (),
        }
    }

    if !stack.is_empty() {
        panic!("Unmatched '[' at position {}", stack.pop().unwrap());
    }

    while instruction_number < instructions_input.len() {
        let instruction = instructions_input[instruction_number];

        match instruction {
            '>' => {
                mem_cell_number += 1;
                if mem_cell_number >= data_tape.len() {
                    panic!("Pointer out of bounds (add)!");
                }
            }
            '<' => {
                if mem_cell_number == 0 {
                    panic!("Pointer out of bounds!");
                }
                mem_cell_number -= 1;
            }
            '+' => {
                data_tape[mem_cell_number] = data_tape[mem_cell_number].wrapping_add(1);
            }
            '-' => {
                data_tape[mem_cell_number] = data_tape[mem_cell_number].wrapping_sub(1);
            }
            '.' => {
                print!("{}", data_tape[mem_cell_number] as char);
            }
            ',' => {
                let mut input = [0u8; 1];
                if io::stdin().read_exact(&mut input).is_ok() {
                    data_tape[mem_cell_number] = input[0];
                }
            }
            '[' => {
                if data_tape[mem_cell_number] == 0 {
                    instruction_number = *loop_pair.get(&instruction_number).unwrap();
                }
            }
            ']' => {
                if data_tape[mem_cell_number] != 0 {
                    instruction_number = *loop_pair.get(&instruction_number).unwrap();
                }
            }
            _ => (),
        }
        instruction_number += 1;
    }
}
