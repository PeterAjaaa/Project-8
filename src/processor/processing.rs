use std::{
    io::{self, Read},
    iter::Peekable,
    slice::Iter,
};

use super::intermediate_representation::Instruction;

/// Generates an IR instruction for the basic operators, excluding the opening and closing bracket operator.
fn ir_generator_helper(inst: char, iter: &mut Peekable<Iter<char>>) -> Option<Instruction> {
    let mut counter = 1;

    while let Some(&&next_inst) = iter.peek() {
        if next_inst == inst {
            counter += 1;
            iter.next();
        } else {
            break;
        }
    }

    match inst {
        '>' => Some(Instruction::FWD(counter)),
        '<' => Some(Instruction::BWD(counter)),
        '+' => Some(Instruction::INC(counter)),
        '-' => Some(Instruction::DEC(counter)),
        '.' => Some(Instruction::OUT(counter)),
        ',' => Some(Instruction::IN(counter)),
        _ => None,
    }
}

/// Generate an IR vec to pass to the processing function
fn generate_ir(instructions_input: &Vec<char>) -> Vec<Instruction> {
    let mut ir_result_vec: Vec<Instruction> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut iter: Peekable<Iter<char>> = instructions_input.iter().peekable();

    while let Some(&instruction) = iter.next() {
        match instruction {
            '>' | '<' | '+' | '-' | '.' | ',' => {
                if let Some(ir_inst) = ir_generator_helper(instruction, &mut iter) {
                    ir_result_vec.push(ir_inst);
                }
            }
            '[' => {
                ir_result_vec.push(Instruction::STARTLOOPJUMP(0));
                let start_pos = ir_result_vec.len() - 1;
                stack.push(start_pos);
            }
            ']' => {
                if let Some(start) = stack.pop() {
                    ir_result_vec.push(Instruction::ENDLOOPJUMP(start));

                    let end_pos = ir_result_vec.len() - 1;

                    if let Some(Instruction::STARTLOOPJUMP(ref mut target)) =
                        ir_result_vec.get_mut(start)
                    {
                        *target = end_pos;
                    }
                }
            }

            _ => (),
        }
    }

    if !stack.is_empty() {
        panic!("Unmatched '[' at position {}", stack.pop().unwrap());
    } else {
        ir_result_vec
    }
}

/// Process the IR instruction and execute it.
pub fn process_ir(data_tape: &mut Vec<u8>, instructions_input: Vec<char>) {
    // Refers to the location or the position of the current instruction in the data tapes
    let mut mem_cell_number: usize = 0;
    // Refers to the instruction individual position.
    let mut instruction_number: usize = 0;

    let ir_vec = generate_ir(&instructions_input);

    while instruction_number < ir_vec.len() {
        let instruction = &ir_vec[instruction_number];

        match instruction {
            Instruction::FWD(val) => {
                mem_cell_number += val;
                if mem_cell_number >= data_tape.len() {
                    panic!("Pointer out of bounds (moving forwards)!");
                }
            }
            Instruction::BWD(val) => {
                if mem_cell_number == 0 {
                    panic!("Pointer out of bounds (moving backwards)!");
                }
                mem_cell_number -= val;
            }
            Instruction::INC(val) => {
                data_tape[mem_cell_number] = data_tape[mem_cell_number].wrapping_add(*val as u8);
            }
            Instruction::DEC(val) => {
                data_tape[mem_cell_number] = data_tape[mem_cell_number].wrapping_sub(*val as u8);
            }
            Instruction::OUT(val) => {
                for _ in 0..*val {
                    print!("{}", data_tape[mem_cell_number] as char);
                }
            }
            Instruction::IN(val) => {
                for _ in 0..*val {
                    let mut input = [0u8; 1];
                    match io::stdin().read_exact(&mut input) {
                        Ok(_) => {
                            data_tape[mem_cell_number] = input[0];
                        }
                        Err(e) => panic!("{}", e),
                    }
                }
            }
            Instruction::STARTLOOPJUMP(target) => {
                if data_tape[mem_cell_number] == 0 {
                    instruction_number = *target;
                }
            }
            Instruction::ENDLOOPJUMP(target) => {
                if data_tape[mem_cell_number] != 0 {
                    instruction_number = *target;
                }
            }
        }
        instruction_number += 1;
    }
}
