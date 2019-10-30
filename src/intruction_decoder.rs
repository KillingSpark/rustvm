use crate::cpu_ops::*;
use crate::{Instruction, Memory, MemoryPointer};

const ADD: u8 = 0;
const SUB: u8 = 1;
const DIV: u8 = 2;
const MUL: u8 = 3;
const LOAD: u8 = 4;
const STORE: u8 = 5;
const HALT: u8 = 6;
const JMP: u8 = 7;
const COND_JMP: u8 = 8;
const LESS: u8 = 9;

pub fn decode_instruction(
    instr_ptr: MemoryPointer,
    mem: &Memory,
) -> Result<Instruction, Box<std::error::Error>> {
    //Read memory at place instr_ptr and decode the op
    //TODO
    let intr_code = mem.mem[instr_ptr as usize];
    match intr_code {
        ADD => {
            let src_left = mem.mem[instr_ptr as usize + 1];
            let src_right = mem.mem[instr_ptr as usize + 2];
            let dst = mem.mem[instr_ptr as usize + 3];
            Ok(Instruction {
                function: make_cpu_op_add(src_left as usize, src_right as usize, dst as usize),
            })
        }
        SUB => {
            let src_left = mem.mem[instr_ptr as usize + 1];
            let src_right = mem.mem[instr_ptr as usize + 2];
            let dst = mem.mem[instr_ptr as usize + 3];
            Ok(Instruction {
                function: make_cpu_op_sub(src_left as usize, src_right as usize, dst as usize),
            })
        }
        DIV => {
            let src_left = mem.mem[instr_ptr as usize + 1];
            let src_right = mem.mem[instr_ptr as usize + 2];
            let dst = mem.mem[instr_ptr as usize + 3];
            Ok(Instruction {
                function: make_cpu_op_div(src_left as usize, src_right as usize, dst as usize),
            })
        }
        MUL => {
            let src_left = mem.mem[instr_ptr as usize + 1];
            let src_right = mem.mem[instr_ptr as usize + 2];
            let dst = mem.mem[instr_ptr as usize + 3];
            Ok(Instruction {
                function: make_cpu_op_mul(src_left as usize, src_right as usize, dst as usize),
            })
        }
        LOAD => {
            let src = mem.mem[instr_ptr as usize + 1];
            let dst = mem.mem[instr_ptr as usize + 2];
            Ok(Instruction {
                function: make_cpu_op_load_8(src as u64, dst as usize),
            })
        }
        STORE => {
            let src = mem.mem[instr_ptr as usize + 1];
            let dst = mem.mem[instr_ptr as usize + 2];
            Ok(Instruction {
                function: make_cpu_op_store_8(src as usize, dst as u64),
            })
        }
        HALT => Ok(Instruction {
            function: make_cpu_op_halt(),
        }),
        JMP => {
            let dst = mem.mem[instr_ptr as usize + 1];
            Ok(Instruction {
                function: make_cpu_op_jmp(dst as u64),
            })
        },
        COND_JMP => {
            let dst = mem.mem[instr_ptr as usize + 1];
            Ok(Instruction {
                function: make_cpu_op_cond_jmp(dst as u64),
            })
        },
        LESS => {
            let src_left = mem.mem[instr_ptr as usize + 1];
            let src_right = mem.mem[instr_ptr as usize + 2];
            Ok(Instruction {
                function: make_cpu_op_less(src_left as usize, src_right as usize    ),
            })
        }
        _ => panic!("Unknown op code"),
    }
}
