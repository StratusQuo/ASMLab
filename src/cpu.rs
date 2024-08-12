use crate::parser::{Instruction, InstructionType, Operand, Register, RegisterDisplayOptions};
use crate::parser::{MemoryDumpOptions, MemoryDumpFormat};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct CPU {
    // General Purpose Registers
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rbp: u64, pub rsp: u64,
    pub r8: u64, pub r9: u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    
    // Special Registers
    pub rip: u64, // Instruction pointer
    pub rflags: u64, // Flags register
    pub cs: u16, // Code Segment
    pub fs: u16, // Extra Segment Register
    pub gs: u16, // Extra Segment Register

    // Individual Flags
    pub cf: bool, // Carry Flag
    pub zf: bool, // Zero Flag
    pub sf: bool, // Sign Flag
    pub of: bool, // Overflow Flag

    // Memory (simple implementation)
    pub memory: Vec<u8>,

    // XMM Registers (for SSE/AVX)
    // 128-Bit XMM Registers (holds 4 doublewords):
    pub xmm: [u128; 16], 
}

// #[derive(Debug, Clone, Copy)]
// pub struct XmmRegister {
//     pub data: [u32; 4],
// }

impl CPU {
    pub fn new() -> Self {
        CPU {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, 
            rsp: 1024 * 1024 - 8,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0,
            rflags: 0x0002, // Default value with bit 1 set (reserved bit)
            cs: 0, fs: 0, gs: 0,
            xmm: [0; 16],
            cf: false, zf: false, sf: false, of: false,
            memory: vec![0; 1024 * 1024], // 1MB of memory
        }
    }

    pub fn get_register_value(&self, register: &Register) -> u64 {
        self[register]
    }

    pub fn format_register_value(&self, register: &Register, options: &RegisterDisplayOptions) -> String {
        let value = self.get_register_value(register);
        if options.human_readable {
            // TODO: Implement your human-readable formatting here (e.g., convert to decimal, signed, etc.)
            format!("{:?} = {}", register, value) // For now, just display the name and value
        } else {
            format!("{:?}: {:#018x}", register, value)
        }
    }

    pub fn dump_memory(&self, options: &MemoryDumpOptions) {
        let address = options.address;
        let size = options.size;

        println!("Memory Dump at 0x{:x}:", address);

        for i in 0..(size / 16) { // Iterate over rows (16 bytes per row)
            print!("0x{:08x}:  ", address + (i * 16) as u64);
            for j in 0..16 { // Iterate over columns
                let index = (address as usize) + (i * 16) + j;
                if let Some(byte) = self.memory.get(index) {
                    match options.format {
                        MemoryDumpFormat::Hex => print!("{:02x} ", byte),
                        MemoryDumpFormat::Decimal => print!("{:3} ", byte),
                    }
                } else {
                    print!("?? "); // Out of bounds
                }
            }
            println!(); // Newline after each row
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        match instruction.instruction_type {
            InstructionType::Mov => self.execute_mov(instruction),
            InstructionType::Add => self.execute_add(instruction),
            InstructionType::Sub => self.execute_sub(instruction),
            InstructionType::And => self.execute_and(instruction),
            InstructionType::Or => self.execute_or(instruction),
            InstructionType::Xor => self.execute_xor(instruction),
            InstructionType::Inc => self.execute_inc(instruction),
            InstructionType::Dec => self.execute_dec(instruction),
            InstructionType::Neg => self.execute_neg(instruction),
            InstructionType::Not => self.execute_not(instruction),
            InstructionType::Shl => self.execute_shl(instruction),
            InstructionType::Shr => self.execute_shr(instruction),
            InstructionType::Rol => self.execute_rol(instruction),
            InstructionType::Ror => self.execute_ror(instruction),
            InstructionType::Push => self.execute_push(instruction),
            InstructionType::Pop => self.execute_pop(instruction),
            InstructionType::Cmp => self.execute_cmp(instruction),
            InstructionType::Test => self.execute_test(instruction),
            InstructionType::Jmp => { self.execute_jmp(instruction); }
            InstructionType::Je => { self.execute_je(instruction); }
            InstructionType::Jne => { self.execute_jne(instruction); }
            InstructionType::Jg => { self.execute_jg(instruction); }
            InstructionType::Jge => { self.execute_jge(instruction); }
            InstructionType::Jl => { self.execute_jl(instruction); }
            InstructionType::Jle => { self.execute_jle(instruction); }
            InstructionType::Call => self.execute_call(instruction),
            InstructionType::Ret => self.execute_ret(instruction),
            //Advanced:
            InstructionType::Paddd => self.execute_paddd(instruction),
            // Bit-Scan Forward:
            InstructionType::Bsf => self.execute_bsf(instruction), 
            InstructionType::Cmovne => self.execute_cmovne(instruction),
            //_ => println!("Unsupported instruction: {:?}", instruction.instruction_type),
        }
        self.rip += 1; // Increment instruction pointer
    }

    fn execute_mov(&mut self, instruction: &Instruction) {
        if let (Operand::Register(dest), Operand::Immediate(imm)) = (&instruction.operands[0], &instruction.operands[1]) {
            self[dest] = *imm as u64;
        } else if let (Operand::Register(dest), Operand::Register(src)) = (&instruction.operands[0], &instruction.operands[1]) {
            self[dest] = self[src];
        }
    }

    fn execute_add(&mut self, instruction: &Instruction) {
        if let (Operand::Register(dest), Operand::Immediate(imm)) = (&instruction.operands[0], &instruction.operands[1]) {
            let (result, overflow) = self[dest].overflowing_add(*imm as u64);
            self[dest] = result;
            self.update_flags(result, overflow);
        } else if let (Operand::Register(dest), Operand::Register(src)) = (&instruction.operands[0], &instruction.operands[1]) {
            let (result, overflow) = self[dest].overflowing_add(self[src]);
            self[dest] = result;
            self.update_flags(result, overflow);
        }
    }

    fn execute_sub(&mut self, instruction: &Instruction) {
        if let (Operand::Register(dest), Operand::Immediate(imm)) = (&instruction.operands[0], &instruction.operands[1]) {
            let (result, overflow) = self[dest].overflowing_sub(*imm as u64);
            self[dest] = result;
            self.update_flags(result, overflow);
        } else if let (Operand::Register(dest), Operand::Register(src)) = (&instruction.operands[0], &instruction.operands[1]) {
            let (result, overflow) = self[dest].overflowing_sub(self[src]);
            self[dest] = result;
            self.update_flags(result, overflow);
        }
    }

    fn execute_and(&mut self, instruction: &Instruction) {
        if let (Operand::Register(dest), Operand::Immediate(imm)) = (&instruction.operands[0], &instruction.operands[1]) {
            self[dest] &= *imm as u64;
            self.update_flags(self[dest], false);
        } else if let (Operand::Register(dest), Operand::Register(src)) = (&instruction.operands[0], &instruction.operands[1]) {
            self[dest] &= self[src];
            self.update_flags(self[dest], false);
        }
    }

    fn execute_or(&mut self, instruction: &Instruction) {
        if let (Operand::Register(dest), Operand::Immediate(imm)) = (&instruction.operands[0], &instruction.operands[1]) {
            self[dest] |= *imm as u64;
            self.update_flags(self[dest], false);
        } else if let (Operand::Register(dest), Operand::Register(src)) = (&instruction.operands[0], &instruction.operands[1]) {
            self[dest] |= self[src];
            self.update_flags(self[dest], false);
        }
    }

    fn execute_xor(&mut self, instruction: &Instruction) {
        if let (Operand::Register(dest), Operand::Immediate(imm)) = (&instruction.operands[0], &instruction.operands[1]) {
            self[dest] ^= *imm as u64;
            self.update_flags(self[dest], false);
        } else if let (Operand::Register(dest), Operand::Register(src)) = (&instruction.operands[0], &instruction.operands[1]) {
            self[dest] ^= self[src];
            self.update_flags(self[dest], false);
        }
    }

    fn execute_inc(&mut self, instruction: &Instruction) {
        if let Operand::Register(reg) = &instruction.operands[0] {
            let (result, overflow) = self[reg].overflowing_add(1);
            self[reg] = result;
            self.update_flags(result, overflow);
        }
    }

    fn execute_dec(&mut self, instruction: &Instruction) {
        if let Operand::Register(reg) = &instruction.operands[0] {
            let (result, overflow) = self[reg].overflowing_sub(1);
            self[reg] = result;
            self.update_flags(result, overflow);
        }
    }

    fn execute_neg(&mut self, instruction: &Instruction) {
        if let Operand::Register(reg) = &instruction.operands[0] {
            let (result, overflow) = (0u64).overflowing_sub(self[reg]);
            self[reg] = result;
            self.update_flags(result, overflow);
        }
    }

    fn execute_not(&mut self, instruction: &Instruction) {
        if let Operand::Register(reg) = &instruction.operands[0] {
            self[reg] = !self[reg];
            self.update_flags(self[reg], false);
        }
    }

    fn execute_shl(&mut self, instruction: &Instruction) {
        if let (Operand::Register(reg), Operand::Immediate(shift)) = (&instruction.operands[0], &instruction.operands[1]) {
            let result = self[reg] << shift;
            self[reg] = result;
            self.update_flags(result, false);
        }
    }

    fn execute_shr(&mut self, instruction: &Instruction) {
        if let (Operand::Register(reg), Operand::Immediate(shift)) = (&instruction.operands[0], &instruction.operands[1]) {
            let result = self[reg] >> shift;
            self[reg] = result;
            self.update_flags(result, false);
        }
    }

    fn execute_rol(&mut self, instruction: &Instruction) {
        if let (Operand::Register(reg), Operand::Immediate(shift)) = (&instruction.operands[0], &instruction.operands[1]) {
            let result = self[reg].rotate_left(*shift as u32);
            self[reg] = result;
            self.update_flags(result, false);
        }
    }

    fn execute_ror(&mut self, instruction: &Instruction) {
        if let (Operand::Register(reg), Operand::Immediate(shift)) = (&instruction.operands[0], &instruction.operands[1]) {
            let result = self[reg].rotate_right(*shift as u32);
            self[reg] = result;
            self.update_flags(result, false);
        }
    }

    fn execute_push(&mut self, instruction: &Instruction) {
        if let Operand::Register(reg) = &instruction.operands[0] {
            self.rsp -= 8;
            let value = self[reg];
            self.write_memory(self.rsp, value);
        }
    }

    fn execute_pop(&mut self, instruction: &Instruction) {
        if let Operand::Register(reg) = &instruction.operands[0] {
            let value = self.read_memory(self.rsp);
            self[reg] = value;
            self.rsp += 8;
        }
    }

    fn execute_cmp(&mut self, instruction: &Instruction) {
        if let (Operand::Register(reg), Operand::Immediate(imm)) = (&instruction.operands[0], &instruction.operands[1]) {
            let (result, overflow) = self[reg].overflowing_sub(*imm as u64);
            self.update_flags(result, overflow);
        } else if let (Operand::Register(reg1), Operand::Register(reg2)) = (&instruction.operands[0], &instruction.operands[1]) {
            let (result, overflow) = self[reg1].overflowing_sub(self[reg2]);
            self.update_flags(result, overflow);
        }
    }

    fn execute_test(&mut self, instruction: &Instruction) {
        if let (Operand::Register(reg), Operand::Immediate(imm)) = (&instruction.operands[0], &instruction.operands[1]) {
            let result = self[reg] & (*imm as u64);
            self.update_flags(result, false);
        } else if let (Operand::Register(reg1), Operand::Register(reg2)) = (&instruction.operands[0], &instruction.operands[1]) {
            let result = self[reg1] & self[reg2];
            self.update_flags(result, false);
        }
    }

    fn execute_jmp(&mut self, instruction: &Instruction) {
        if let Operand::Immediate(target) = instruction.operands[0] {
            self.rip = target as u64 - 1; // -1 because rip is incremented after execution
        }
    }
    
    fn execute_je(&mut self, instruction: &Instruction) {
        if self.zf {
            self.execute_jmp(instruction);
        } else {
            // Do nothing if condition is not met
        }
    }

    fn execute_jne(&mut self, instruction: &Instruction) {
        if !self.zf {
            self.execute_jmp(instruction);
        } else {
            // Do nothing if condition is not met
        }
    }

    fn execute_jg(&mut self, instruction: &Instruction) {
        if !self.zf && self.sf == self.of {
            self.execute_jmp(instruction);
        } else {
            // Do nothing if condition is not met
        }
    }

    fn execute_jge(&mut self, instruction: &Instruction) {
        if self.sf == self.of {
            self.execute_jmp(instruction);
        } else {
            // Do nothing if condition is not met
        }
    }

    fn execute_jl(&mut self, instruction: &Instruction) {
        if self.sf != self.of {
            self.execute_jmp(instruction);
        } else {
            // Do nothing if condition is not met
        }
    }

    fn execute_jle(&mut self, instruction: &Instruction) {
        if self.zf || self.sf != self.of {
            self.execute_jmp(instruction);
        } else {
            // Do nothing if condition is not met
        }
    }

    fn execute_call(&mut self, instruction: &Instruction) {
        self.rsp -= 8;
        self.write_memory(self.rsp, self.rip + 1);
        self.execute_jmp(instruction);
    }

    fn execute_ret(&mut self, _instruction: &Instruction) {
        self.rip = self.read_memory(self.rsp) - 1; // -1 because rip is incremented after execution
        self.rsp += 8;
    }

    fn execute_bsf(&mut self, instruction: &Instruction) {
        if let (Operand::Register(dest), Operand::Register(src)) = (&instruction.operands[0], &instruction.operands[1]) {
            let source_value = self[src];
            if source_value == 0 {
                self.zf = true; // Set ZF if source is zero
            } else {
                self.zf = false;
                let mut index = 0;
                while (source_value & (1 << index)) == 0 { // Find the index of the first set bit
                    index += 1;
                }
                self[dest] = index; 
            }
        } else {
            println!("Invalid operands for BSF instruction");
        }
    }

    fn execute_cmovne(&mut self, instruction: &Instruction) {
        if !self.zf { // Execute only if ZF is not set (not equal)
            if let (Operand::Register(dest), Operand::Register(src)) = 
                (&instruction.operands[0], &instruction.operands[1]) 
            {
                self[dest] = self[src];
            } else {
                println!("Invalid operands for CMOVNE instruction");
            }
        }
    }

    fn execute_paddd(&mut self, instruction: &Instruction) {
        if let (Operand::XmmRegister(dest), Operand::XmmRegister(src)) = 
            (&instruction.operands[0], &instruction.operands[1])
        {
            let dest_val = self.xmm[*dest as usize];
            let src_val = self.xmm[*src as usize];
            let result = (0..4).map(|i| {
                let dest_part = (dest_val >> (i * 32)) & 0xFFFFFFFF;
                let src_part = (src_val >> (i * 32)) & 0xFFFFFFFF;
                (dest_part.wrapping_add(src_part) & 0xFFFFFFFF) << (i * 32)
            }).fold(0, |acc, x| acc | x);
            self.xmm[*dest as usize] = result;
        } else {
            println!("Invalid operands for paddd instruction");
        }
    }

    fn read_memory(&self, address: u64) -> u64 {
        let bytes = &self.memory[address as usize..address as usize + 8];
        u64::from_le_bytes(bytes.try_into().unwrap())
    }

    fn write_memory(&mut self, address: u64, value: u64) {
        let bytes = value.to_le_bytes();
        self.memory[address as usize..address as usize + 8].copy_from_slice(&bytes);
    }

    // Implement other instruction executions (or, xor, inc, dec, etc.) similarly...

    fn update_flags(&mut self, result: u64, overflow: bool) {
        self.zf = result == 0;
        self.sf = (result as i64) < 0;
        self.cf = result < self[&Register::Rax]; // This is a simplification, carry should be set based on the operation
        self.of = overflow;
        
        // Update rflags register
        self.rflags = (self.cf as u64) |
                      ((self.zf as u64) << 6) |
                      ((self.sf as u64) << 7) |
                      ((self.of as u64) << 11);
    }
}

impl Index<&Register> for CPU {
    type Output = u64;

    fn index(&self, register: &Register) -> &Self::Output {
        match register {
            Register::Rax => &self.rax,
            Register::Rbx => &self.rbx,
            Register::Rcx => &self.rcx,
            Register::Rdx => &self.rdx,
            Register::Rsi => &self.rsi,
            Register::Rdi => &self.rdi,
            Register::Rbp => &self.rbp,
            Register::Rsp => &self.rsp,
            Register::R8 => &self.r8,
            Register::R9 => &self.r9,
            Register::R10 => &self.r10,
            Register::R11 => &self.r11,
            Register::R12 => &self.r12,
            Register::R13 => &self.r13,
            Register::R14 => &self.r14,
            Register::R15 => &self.r15,
        }
    }
}

impl IndexMut<&Register> for CPU {
    fn index_mut(&mut self, register: &Register) -> &mut Self::Output {
        match register {
            Register::Rax => &mut self.rax,
            Register::Rbx => &mut self.rbx,
            Register::Rcx => &mut self.rcx,
            Register::Rdx => &mut self.rdx,
            Register::Rsi => &mut self.rsi,
            Register::Rdi => &mut self.rdi,
            Register::Rbp => &mut self.rbp,
            Register::Rsp => &mut self.rsp,
            Register::R8 => &mut self.r8,
            Register::R9 => &mut self.r9,
            Register::R10 => &mut self.r10,
            Register::R11 => &mut self.r11,
            Register::R12 => &mut self.r12,
            Register::R13 => &mut self.r13,
            Register::R14 => &mut self.r14,
            Register::R15 => &mut self.r15,
        }
    }
}