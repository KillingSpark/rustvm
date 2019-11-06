mod register_def;
use register_def::*;
mod cpu_ops;
mod intruction_decoder;
use intruction_decoder::decode_instruction;

use std::collections::HashMap;
use std::time;

type MemoryPointer = u64;

#[derive(Clone, Default, Debug)]
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
    regs: [u64; NUM_REGISTERS],
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

fn load_program() -> Vec<u8> {
    let mut mem = vec![0u8; 22];

    //Add X + Y -> X
    mem[0] = crate::intruction_decoder::ADD;
    mem[1] = REG_X as u8;
    mem[2] = REG_Y as u8;
    mem[3] = REG_X as u8;

    //Save to memory
    mem[4] = crate::intruction_decoder::STORE;
    mem[5] = REG_X as u8;
    mem[6] = REG_A as u8;

    //Load from memory
    mem[7] = crate::intruction_decoder::LOAD;
    mem[8] = REG_A as u8;
    mem[9] = REG_B as u8;

    //test reg[0] < reg[4]
    mem[10] = crate::intruction_decoder::LESS;
    mem[11] = REG_B as u8;
    mem[12] = REG_Z as u8;

    //change program to use REG_D instead of REG_A
    mem[13] = crate::intruction_decoder::STORE8;
    mem[14] = REG_E as u8; //contains REG_D as value
    mem[15] = REG_F as u8; //contains 6

    mem[16] = crate::intruction_decoder::STORE8;
    mem[17] = REG_E as u8; //contains REG_D as value
    mem[18] = REG_G as u8; // contains 8

    //jmp to start if yes
    mem[19] = crate::intruction_decoder::COND_JMP;
    mem[20] = 0;

    //halt
    mem[21] = crate::intruction_decoder::HALT;

    mem
}

fn main() {
    let mut instruction_cache: HashMap<MemoryPointer, CacheEntry<Instruction>> = HashMap::new();

    let mut cpu_state = CPUState::default();
    cpu_state.regs[REG_X] = 0;
    cpu_state.regs[REG_Y] = 1;

    // if reg[1] get bigger than this the machine halts
    cpu_state.regs[REG_Z] = 1_000;

    // memory addr where to save the value
    cpu_state.regs[REG_A] = 1015;
    cpu_state.regs[REG_D] = 1024;

    // for modifying the code to change A to D
    cpu_state.regs[REG_E] = REG_D as u64;
    cpu_state.regs[REG_F] = 6;
    cpu_state.regs[REG_G] = 8;

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

    //TODO
    //read elf file
    //initialize memory
    //initialize io, etc. pp.
    //setup cpu state

    let mem_img = load_program();
    for x in 0..mem_img.len() {
        vm_state.mem.set(x, mem_img[x]);
    }

    let start = time::Instant::now();
    while !vm_state.stop_execution {
        let instr: &mut Instruction = match instruction_cache.get_mut(&vm_state.cpu.instr_ptr()) {
            Some(i) => {
                if i.valid {
                    //if set_counter got increased we need to load again
                    println!(
                        "ptr: {}, cache: {}, memory: {}",
                        vm_state.cpu.instr_ptr(),
                        i.set_counter_when_cached,
                        vm_state.mem.mem[vm_state.cpu.instr_ptr() as usize].set_counter
                    );

                    let need_reload = (i.set_counter_when_cached
                        < vm_state.mem.mem[vm_state.cpu.instr_ptr() as usize].set_counter)
                        | (i.set_counter_when_cached
                            < vm_state.mem.mem[vm_state.cpu.instr_ptr() as usize + 1].set_counter)
                        | (i.set_counter_when_cached
                            < vm_state.mem.mem[vm_state.cpu.instr_ptr() as usize + 2].set_counter);

                    if need_reload {
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
    println!(
        "End memory [1015..1023]: {:?}",
        &vm_state.mem.mem[1015..1023]
            .iter()
            .map(|x| x.value)
            .collect::<Vec<_>>()
    );
    println!(
        "End memory [1024..1031]: {:?}",
        &vm_state.mem.mem[1024..1031]
            .iter()
            .map(|x| x.value)
            .collect::<Vec<_>>()
    );
    println!("Took {} milliseconds", start.elapsed().as_millis());
}
