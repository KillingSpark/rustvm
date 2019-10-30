mod register_def;
use register_def::*;
mod cpu_ops;
mod intruction_decoder;
use intruction_decoder::decode_instruction;

use std::collections::HashMap;
use std::time;

type MemoryPointer = u64;

#[derive(Clone, Default)]
pub struct Memory {
    mem: Vec<u8>
}

#[derive(Default)]
pub struct VMState{
    cpu: CPUState,
    mem: Memory,
    stop_execution: bool,
}

#[derive(Default, Debug)]
pub struct CPUState {
    regs: [u64; 5],
    cmp_flag: bool,
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

fn main() {

    let mut instruction_cache: HashMap<MemoryPointer, CacheEntry<Instruction>> = HashMap::new();

    let mut cpu_state = CPUState::default();
    cpu_state.regs[1] = 0;
    cpu_state.regs[2] = 1;
    cpu_state.regs[4] = 1_000_000;

    let mut vm_state = VMState{
        mem: Memory{
            mem: vec![0;1024*1024]
        },
        cpu: cpu_state,
        stop_execution: false,
    };

    //Add reg[1] + reg[2] -> reg[1]
    vm_state.mem.mem[0] = 0;
    vm_state.mem.mem[1] = 1;
    vm_state.mem.mem[2] = 2;
    vm_state.mem.mem[3] = 1;

    //test reg[0] < reg[4]
    vm_state.mem.mem[4] = 9;
    vm_state.mem.mem[5] = 1;
    vm_state.mem.mem[6] = 4;

    //jmp to start if yes
    vm_state.mem.mem[7] = 8;
    vm_state.mem.mem[8] = 0;

    //halt
    vm_state.mem.mem[9] = 6;

    //TODO
    //read elf file
    //initialize memory
    //initialize io, etc. pp.
    //setup cpu state

    let start = time::Instant::now();
    while !vm_state.stop_execution {
        let instr: &mut Instruction = match instruction_cache.get_mut(&vm_state.cpu.instr_ptr()) {
            Some(i) => {
                if i.valid {
                    &mut i.value
                } else {
                    instruction_cache.insert(
                        vm_state.cpu.instr_ptr(),
                        CacheEntry {
                            value: decode_instruction(vm_state.cpu.instr_ptr(), &vm_state.mem).unwrap(),
                            valid: true,
                        },
                    );
                    &mut instruction_cache
                        .get_mut(&vm_state.cpu.instr_ptr())
                        .unwrap()
                        .value
                }
            }
            None => {
                //decode new instruction
                instruction_cache.insert(
                    vm_state.cpu.instr_ptr(),
                    CacheEntry {
                        value: decode_instruction(vm_state.cpu.instr_ptr(), &vm_state.mem).unwrap(),
                        valid: true,
                    },
                );
                &mut instruction_cache
                    .get_mut(&vm_state.cpu.instr_ptr())
                    .unwrap()
                    .value
            }
        };

        match (instr.function)(&mut vm_state) {
            Ok(()) => { /* Happy */ }
            Err(e) => panic!(e.description().to_owned()),
        };
    }

    println!("End cpu state: {:?}", vm_state.cpu);
    println!("Took {} milliseconds", start.elapsed().as_millis());
}
