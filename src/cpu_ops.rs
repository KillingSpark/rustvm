#![allow(dead_code)]

use crate::register_def::*;
use crate::CPUOpFn;
use crate::Memory;
use crate::VMState;

pub fn make_cpu_op_add(left_src: usize, right_src: usize, dst: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.cpu.regs[dst] = vm_state.cpu.regs[left_src] + vm_state.cpu.regs[right_src];
        vm_state.cpu.regs[REG_INSTR_PTR] += 4;
        Ok(())
    })
}

pub fn make_cpu_op_sub(left_src: usize, right_src: usize, dst: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.cpu.regs[dst] = vm_state.cpu.regs[left_src] - vm_state.cpu.regs[right_src];
        vm_state.cpu.regs[REG_INSTR_PTR] += 4;
        Ok(())
    })
}

pub fn make_cpu_op_div(left_src: usize, right_src: usize, dst: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.cpu.regs[dst] = vm_state.cpu.regs[left_src] / vm_state.cpu.regs[right_src];
        vm_state.cpu.regs[REG_INSTR_PTR] += 4;
        Ok(())
    })
}

pub fn make_cpu_op_mul(left_src: usize, right_src: usize, dst: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.cpu.regs[dst] = vm_state.cpu.regs[left_src] * vm_state.cpu.regs[right_src];
        vm_state.cpu.regs[REG_INSTR_PTR] += 4;
        Ok(())
    })
}

pub fn make_cpu_op_less(left_src: usize, right_src: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.cpu.cmp_flag = vm_state.cpu.regs[left_src] < vm_state.cpu.regs[right_src];
        vm_state.cpu.regs[REG_INSTR_PTR] += 3;
        Ok(())
    })
}

fn store_64(mem: &mut Memory, dst_mem: usize, val: u64) {
    mem.set(dst_mem as usize, (val & 0xFF) as u8);
    mem.set(dst_mem as usize + 1, ((val >> 8) & 0xFF) as u8);
    mem.set(dst_mem as usize + 2, ((val >> 16) & 0xFF) as u8);
    mem.set(dst_mem as usize + 3, ((val >> 24) & 0xFF) as u8);
    mem.set(dst_mem as usize + 4, ((val >> 32) & 0xFF) as u8);
    mem.set(dst_mem as usize + 5, ((val >> 40) & 0xFF) as u8);
    mem.set(dst_mem as usize + 6, ((val >> 48) & 0xFF) as u8);
    mem.set(dst_mem as usize + 7, ((val >> 56) & 0xFF) as u8);
}

fn load_64(mem: &Memory, src_mem: usize) -> u64 {
    let mut val = mem.get(src_mem as usize) as u64;
    val |= (mem.get(src_mem as usize + 1) as u64) << 8;
    val |= (mem.get(src_mem as usize + 2) as u64) << 16;
    val |= (mem.get(src_mem as usize + 3) as u64) << 24;
    val |= (mem.get(src_mem as usize + 4) as u64) << 32;
    val |= (mem.get(src_mem as usize + 5) as u64) << 40;
    val |= (mem.get(src_mem as usize + 6) as u64) << 48;
    val |= (mem.get(src_mem as usize + 7) as u64) << 56;

    val
}

pub fn make_cpu_op_store_64(src_reg: usize, dst_mem_reg: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        let mem_base = vm_state.cpu.regs[dst_mem_reg] as usize;
        store_64(&mut vm_state.mem, mem_base, vm_state.cpu.regs[src_reg]);

        vm_state.cpu.regs[REG_INSTR_PTR] += 3;
        Ok(())
    })
}
pub fn make_cpu_op_store_8(src_reg: usize, dst_mem_reg: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        let mem_base = vm_state.cpu.regs[dst_mem_reg] as usize;
        vm_state
            .mem
            .set(mem_base, (vm_state.cpu.regs[src_reg] & 0xFF) as u8);

        vm_state.cpu.regs[REG_INSTR_PTR] += 3;
        Ok(())
    })
}

pub fn make_cpu_op_load_8(src_mem_reg: usize, dst_reg: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        let mem_base = vm_state.cpu.regs[src_mem_reg] as usize;
        vm_state.cpu.regs[dst_reg] = vm_state.mem.get(mem_base) as u64;

        vm_state.cpu.regs[REG_INSTR_PTR] += 3;
        Ok(())
    })
}
pub fn make_cpu_op_load_64(src_mem_reg: usize, dst_reg: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        let mem_base = vm_state.cpu.regs[src_mem_reg] as usize;
        vm_state.cpu.regs[dst_reg] = load_64(&vm_state.mem, mem_base);

        vm_state.cpu.regs[REG_INSTR_PTR] += 3;
        Ok(())
    })
}

pub fn make_cpu_op_halt() -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.stop_execution = true;
        Ok(())
    })
}

pub fn make_cpu_op_jmp(dst_mem: u64) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.cpu.regs[REG_INSTR_PTR] = dst_mem;
        Ok(())
    })
}

pub fn make_cpu_op_cond_jmp(dst_mem: u64) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        if vm_state.cpu.cmp_flag {
            vm_state.cpu.regs[REG_INSTR_PTR] = dst_mem;
        } else {
            vm_state.cpu.regs[REG_INSTR_PTR] += 2;
        }
        Ok(())
    })
}

pub fn make_cpu_op_push_state_to_stack() -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        let dst = vm_state.cpu.regs[REG_STACK_PTR] as usize;
        let mut current_mem_addr = dst;

        //save registers
        for reg in &vm_state.cpu.regs {
            store_64(&mut vm_state.mem, current_mem_addr, *reg);
            current_mem_addr += 8;
        }

        //save cmp result
        if vm_state.cpu.cmp_flag {
            vm_state.mem.set(current_mem_addr, 1);
        } else {
            vm_state.mem.set(current_mem_addr, 0);
        }
        current_mem_addr += 1;

        vm_state.cpu.regs[REG_STACK_PTR] = current_mem_addr as u64;

        Ok(())
    })
}

pub fn make_cpu_op_load_state_from_stack() -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        let dst = vm_state.cpu.regs[REG_STACK_PTR] as usize;
        let mut current_mem_addr = dst;

        //load registers
        for idx in 0..vm_state.cpu.regs.len() {
            vm_state.cpu.regs[idx] = load_64(&mut vm_state.mem, current_mem_addr);
            current_mem_addr += 8;
        }

        //load cmp result
        vm_state.cpu.cmp_flag = vm_state.mem.get(current_mem_addr) != 0;

        Ok(())
    })
}
