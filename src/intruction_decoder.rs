use crate::cpu_ops::*;
use crate::{Instruction, Memory, MemoryPointer};

pub const ADD: u8 = 0;
pub const SUB: u8 = 1;
pub const DIV: u8 = 2;
pub const MUL: u8 = 3;
pub const LOAD: u8 = 4;
pub const STORE: u8 = 5;
pub const HALT: u8 = 6;
pub const JMP: u8 = 7;
pub const COND_JMP: u8 = 8;
pub const LESS: u8 = 9;
pub const PUSH_STACK: u8 = 10;
pub const POP_STACK: u8 = 11;
pub const STORE8: u8 = 12;
pub const LOAD_IMMEDIATE: u8 = 13;

pub fn decode_instruction(
    instr_ptr: MemoryPointer,
    mem: &Memory,
) -> Result<Instruction, Box<std::error::Error>> {
    //Read memory at place instr_ptr and decode the op
    //TODO
    let intr_code = mem.get(instr_ptr as usize);
    match intr_code {
        ADD => {
            let src_left = mem.get(instr_ptr as usize + 1);
            let src_right = mem.get(instr_ptr as usize + 2);
            let dst = mem.get(instr_ptr as usize + 3);
            Ok(Instruction {
                function: make_cpu_op_add(src_left as usize, src_right as usize, dst as usize),
            })
        }
        SUB => {
            let src_left = mem.get(instr_ptr as usize + 1);
            let src_right = mem.get(instr_ptr as usize + 2);
            let dst = mem.get(instr_ptr as usize + 3);
            Ok(Instruction {
                function: make_cpu_op_sub(src_left as usize, src_right as usize, dst as usize),
            })
        }
        DIV => {
            let src_left = mem.get(instr_ptr as usize + 1);
            let src_right = mem.get(instr_ptr as usize + 2);
            let dst = mem.get(instr_ptr as usize + 3);
            Ok(Instruction {
                function: make_cpu_op_div(src_left as usize, src_right as usize, dst as usize),
            })
        }
        MUL => {
            let src_left = mem.get(instr_ptr as usize + 1);
            let src_right = mem.get(instr_ptr as usize + 2);
            let dst = mem.get(instr_ptr as usize + 3);
            Ok(Instruction {
                function: make_cpu_op_mul(src_left as usize, src_right as usize, dst as usize),
            })
        }
        LOAD => {
            let src = mem.get(instr_ptr as usize + 1);
            let dst = mem.get(instr_ptr as usize + 2);
            Ok(Instruction {
                function: make_cpu_op_load_64(src as usize, dst as usize),
            })
        }
        STORE => {
            let src = mem.get(instr_ptr as usize + 1);
            let dst = mem.get(instr_ptr as usize + 2);
            Ok(Instruction {
                function: make_cpu_op_store_64(src as usize, dst as usize),
            })
        }
        STORE8 => {
            let src = mem.get(instr_ptr as usize + 1);
            let dst = mem.get(instr_ptr as usize + 2);
            Ok(Instruction {
                function: make_cpu_op_store_8(src as usize, dst as usize),
            })
        }
        HALT => Ok(Instruction {
            function: make_cpu_op_halt(),
        }),
        JMP => {
            let dst = mem.get(instr_ptr as usize + 1);
            Ok(Instruction {
                function: make_cpu_op_jmp(dst as u64),
            })
        }
        COND_JMP => {
            let dst = mem.get(instr_ptr as usize + 1);
            Ok(Instruction {
                function: make_cpu_op_cond_jmp(dst as u64),
            })
        }
        LESS => {
            let src_left = mem.get(instr_ptr as usize + 1);
            let src_right = mem.get(instr_ptr as usize + 2);
            Ok(Instruction {
                function: make_cpu_op_less(src_left as usize, src_right as usize),
            })
        }
        PUSH_STACK => Ok(Instruction {
            function: make_cpu_op_push_state_to_stack(),
        }),
        POP_STACK => Ok(Instruction {
            function: make_cpu_op_load_state_from_stack(),
        }),
        LOAD_IMMEDIATE => {
            let dst_reg = mem.get(instr_ptr as usize + 1);
            let value = mem.get(instr_ptr as usize + 2);

            Ok(Instruction {
                function: make_cpu_op_load_immediate(dst_reg as usize, value as u64),
            })
        }
        _ => panic!("Unknown op code"),
    }
}
