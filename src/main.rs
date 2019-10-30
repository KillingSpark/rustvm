mod register_def;
use register_def::*;
mod cpu_ops;
mod intruction_decoder;
use intruction_decoder::decode_instruction;

use std::collections::HashMap;
use std::time;

type MemoryPointer = u64;

#[derive(Clone, Default)]
struct MemoryEntry {
    value: u8,
    set_counter: u64,
}

#[derive(Clone, Default)]
pub struct Memory {
    mem: Vec<MemoryEntry>,
}

impl Memory {
    pub fn set(&mut self, addr: usize, value: u8) {
        self.mem[addr].value = value;
        self.mem[addr].set_counter += 1;
    }
    pub fn get(&self, addr: usize) -> u8 {
        self.mem[addr].value
    }
}

#[derive(Default)]
pub struct VMState {
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

type CPUOpFn = Fn(&mut VMState) -> Result<(), Box<std::error::Error>>;

pub struct Instruction {
    function: Box<CPUOpFn>,
}

struct CacheEntry<T> {
    value: T,
    valid: bool,
    set_counter_when_cached: u64,
}

fn main() {
    let mut instruction_cache: HashMap<MemoryPointer, CacheEntry<Instruction>> = HashMap::new();

    let mut cpu_state = CPUState::default();
    cpu_state.regs[1] = 0;
    cpu_state.regs[2] = 1;
    
    // if reg[1] get bigger than this the machine halts
    cpu_state.regs[4] = 1_000_000_000;

    let mut vm_state = VMState {
        mem: Memory {
            mem: vec![
                MemoryEntry {
                    value: 0,
                    set_counter: 0
                };
                1024 * 1024
            ],
        },
        cpu: cpu_state,
        stop_execution: false,
    };

    //Add reg[1] + reg[2] -> reg[1]
    vm_state.mem.mem[0].value = 0;
    vm_state.mem.mem[1].value = 1;
    vm_state.mem.mem[2].value = 2;
    vm_state.mem.mem[3].value = 1;

    //test reg[0] < reg[4]
    vm_state.mem.mem[4].value = 9;
    vm_state.mem.mem[5].value = 1;
    vm_state.mem.mem[6].value = 4;

    //jmp to start if yes
    vm_state.mem.mem[7].value = 8;
    vm_state.mem.mem[8].value = 0;

    //halt
    vm_state.mem.mem[9].value = 6;

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
                    //if set_counter got increased we need to load again
                    if i.set_counter_when_cached
                        < vm_state.mem.mem[vm_state.cpu.instr_ptr() as usize].set_counter
                    {
                        instruction_cache.insert(
                            vm_state.cpu.instr_ptr(),
                            CacheEntry {
                                value: decode_instruction(vm_state.cpu.instr_ptr(), &vm_state.mem)
                                    .unwrap(),
                                valid: true,
                                set_counter_when_cached: vm_state.mem.mem
                                    [vm_state.cpu.instr_ptr() as usize]
                                    .set_counter,
                            },
                        );
                        &mut instruction_cache
                            .get_mut(&vm_state.cpu.instr_ptr())
                            .unwrap()
                            .value
                    } else {
                        //here all is good and the cached value can be used
                        &mut i.value
                    }
                } else {
                    //cache entry was invalidated by something need to reload
                    instruction_cache.insert(
                        vm_state.cpu.instr_ptr(),
                        CacheEntry {
                            value: decode_instruction(vm_state.cpu.instr_ptr(), &vm_state.mem)
                                .unwrap(),
                            valid: true,
                            set_counter_when_cached: vm_state.mem.mem
                                [vm_state.cpu.instr_ptr() as usize]
                                .set_counter,
                        },
                    );
                    &mut instruction_cache
                        .get_mut(&vm_state.cpu.instr_ptr())
                        .unwrap()
                        .value
                }
            }
            None => {
                //decode instruction that has not been cached yet
                instruction_cache.insert(
                    vm_state.cpu.instr_ptr(),
                    CacheEntry {
                        value: decode_instruction(vm_state.cpu.instr_ptr(), &vm_state.mem).unwrap(),
                        valid: true,
                        set_counter_when_cached: vm_state.mem.mem
                            [vm_state.cpu.instr_ptr() as usize]
                            .set_counter,
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
