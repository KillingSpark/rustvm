#![allow(dead_code)]

use crate::CPUOpFn;
use crate::VMState;


pub fn make_cpu_op_add(left_src: usize, right_src: usize, dst: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.cpu.regs[dst] = vm_state.cpu.regs[left_src] + vm_state.cpu.regs[right_src];
        Ok(())
    })
}

pub fn make_cpu_op_write_back_64(src_reg: usize, dst_mem: u64) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.mem.mem[dst_mem as usize] = (vm_state.cpu.regs[src_reg] & 0xFF) as u8;

        vm_state.mem.mem[(dst_mem + 1) as usize] = ((vm_state.cpu.regs[src_reg] >> 8) & 0xFF) as u8;

        vm_state.mem.mem[(dst_mem + 2) as usize] =
            ((vm_state.cpu.regs[src_reg] >> 16) & 0xFF) as u8;

        vm_state.mem.mem[(dst_mem + 3) as usize] =
            ((vm_state.cpu.regs[src_reg] >> 24) & 0xFF) as u8;

        vm_state.mem.mem[(dst_mem + 4) as usize] =
            ((vm_state.cpu.regs[src_reg] >> 32) & 0xFF) as u8;

        vm_state.mem.mem[(dst_mem + 5) as usize] =
            ((vm_state.cpu.regs[src_reg] >> 40) & 0xFF) as u8;

        vm_state.mem.mem[(dst_mem + 6) as usize] =
            ((vm_state.cpu.regs[src_reg] >> 48) & 0xFF) as u8;

        vm_state.mem.mem[(dst_mem + 7) as usize] =
            ((vm_state.cpu.regs[src_reg] >> 56) & 0xFF) as u8;

        Ok(())
    })
}
pub fn make_cpu_op_write_back_32(src_reg: usize, dst_mem: u64) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.mem.mem[dst_mem as usize] = (vm_state.cpu.regs[src_reg] & 0xFF) as u8;
        vm_state.mem.mem[(dst_mem + 1) as usize] = ((vm_state.cpu.regs[src_reg] >> 8) & 0xFF) as u8;
        vm_state.mem.mem[(dst_mem + 2) as usize] =
            ((vm_state.cpu.regs[src_reg] >> 16) & 0xFF) as u8;
        vm_state.mem.mem[(dst_mem + 3) as usize] =
            ((vm_state.cpu.regs[src_reg] >> 24) & 0xFF) as u8;
        Ok(())
    })
}

pub fn make_cpu_op_write_back_16(src_reg: usize, dst_mem: u64) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.mem.mem[dst_mem as usize] = (vm_state.cpu.regs[src_reg] & 0xFF) as u8;
        vm_state.mem.mem[(dst_mem + 1) as usize] = ((vm_state.cpu.regs[src_reg] >> 8) & 0xFF) as u8;
        Ok(())
    })
}
pub fn make_cpu_op_write_back_8(src_reg: usize, dst_mem: u64) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.mem.mem[dst_mem as usize] = (vm_state.cpu.regs[src_reg] & 0xFF) as u8;
        Ok(())
    })
}

pub fn make_cpu_op_load_8(src_mem: u64, dst_reg: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.cpu.regs[dst_reg] = vm_state.mem.mem[src_mem as usize] as u64;
        Ok(())
    })
}
pub fn make_cpu_op_load_16(src_mem: u64, dst_reg: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.cpu.regs[dst_reg] = vm_state.mem.mem[src_mem as usize] as u64;
        vm_state.cpu.regs[dst_reg] |= (vm_state.mem.mem[src_mem as usize + 1] as u64) << 8;
        Ok(())
    })
}
pub fn make_cpu_op_load_32(src_mem: u64, dst_reg: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.cpu.regs[dst_reg] = vm_state.mem.mem[src_mem as usize] as u64;
        vm_state.cpu.regs[dst_reg] |= (vm_state.mem.mem[src_mem as usize + 1] as u64) << 8;
        vm_state.cpu.regs[dst_reg] |= (vm_state.mem.mem[src_mem as usize + 2] as u64) << 16;
        vm_state.cpu.regs[dst_reg] |= (vm_state.mem.mem[src_mem as usize + 3] as u64) << 24;
        Ok(())
    })
}
pub fn make_cpu_op_load_64(src_mem: u64, dst_reg: usize) -> Box<CPUOpFn> {
    Box::new(move |vm_state: &mut VMState| {
        vm_state.cpu.regs[dst_reg] = vm_state.mem.mem[src_mem as usize] as u64;
        vm_state.cpu.regs[dst_reg] |= (vm_state.mem.mem[src_mem as usize + 1] as u64) << 8;
        vm_state.cpu.regs[dst_reg] |= (vm_state.mem.mem[src_mem as usize + 2] as u64) << 16;
        vm_state.cpu.regs[dst_reg] |= (vm_state.mem.mem[src_mem as usize + 3] as u64) << 24;
        vm_state.cpu.regs[dst_reg] |= (vm_state.mem.mem[src_mem as usize + 4] as u64) << 32;
        vm_state.cpu.regs[dst_reg] |= (vm_state.mem.mem[src_mem as usize + 5] as u64) << 40;
        vm_state.cpu.regs[dst_reg] |= (vm_state.mem.mem[src_mem as usize + 6] as u64) << 48;
        vm_state.cpu.regs[dst_reg] |= (vm_state.mem.mem[src_mem as usize + 7] as u64) << 56;
        Ok(())
    })
}
