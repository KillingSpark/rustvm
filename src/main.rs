mod register_def;
use register_def::*;
mod cpu_ops;

use std::collections::HashMap;
use std::time;

type MemoryPointer = u64;

#[derive(Clone, Default)]
pub struct Memory {
    mem: Vec<u8>
}

#[derive(Default, Clone)]
pub struct VMState{
    cpu: CPUState,
    mem: Memory,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct CPUState {
    regs: [u64; 5],
}

impl CPUState {
    pub fn instr_ptr(&self) -> u64 {
        self.regs[REG_INSTR_PTR]
    }
}

type CPUOpFn =
    Fn(&mut VMState) -> Result<(), Box<std::error::Error>>;

pub struct Instruction {
    function: Box<CPUOpFn>,
}

struct CacheEntry<T> {
    value: T,
    valid: bool,
}



fn decode_instruction(
    instr_ptr: MemoryPointer,
) -> Result<Instruction, Box<std::error::Error>> {
    //Read memory at place instr_ptr and decode the op
    //TODO
    let _ = instr_ptr;
    Ok(Instruction {
        function: cpu_ops::make_cpu_op_add(0, 1, 0),
    })
}

fn main() {
    //TODO
    //read elf file
    //initialize memory
    //initialize io, etc. pp.
    //setup cpu state

    let mut instruction_cache: HashMap<MemoryPointer, CacheEntry<Instruction>> = HashMap::new();

    let mut cpu_state = CPUState::default();
    cpu_state.regs[1] = 1;

    let mut vm_state = VMState{
        mem: Memory::default(),
        cpu: cpu_state,
    };

    let start = time::Instant::now();
    for _ in 0..1_000_000 {
        let instr: &mut Instruction = match instruction_cache.get_mut(&cpu_state.instr_ptr()) {
            Some(i) => {
                if i.valid {
                    &mut i.value
                } else {
                    instruction_cache.insert(
                        cpu_state.instr_ptr(),
                        CacheEntry {
                            value: decode_instruction(cpu_state.instr_ptr()).unwrap(),
                            valid: true,
                        },
                    );
                    &mut instruction_cache
                        .get_mut(&cpu_state.instr_ptr())
                        .unwrap()
                        .value
                }
            }
            None => {
                //decode new instruction
                instruction_cache.insert(
                    cpu_state.instr_ptr(),
                    CacheEntry {
                        value: decode_instruction(cpu_state.instr_ptr()).unwrap(),
                        valid: true,
                    },
                );
                &mut instruction_cache
                    .get_mut(&cpu_state.instr_ptr())
                    .unwrap()
                    .value
            }
        };

        match (instr.function)(&mut vm_state) {
            Ok(()) => { /* Happy */ }
            Err(e) => panic!(e.description().to_owned()),
        };
    }

    println!("Took {} milliseconds", start.elapsed().as_millis());
}
