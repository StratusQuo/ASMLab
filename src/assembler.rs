use iced_x86::code_asm::{AsmRegister64, CodeAssembler};
use iced_x86::code_asm::registers::xmm;
use iced_x86::Register;
use crate::parser::{Instruction, InstructionType, Operand, Register as ParserRegister};

pub fn assemble_instruction(instruction: &Instruction) -> Result<Vec<u8>, String> {
    let mut assembler = CodeAssembler::new(64).map_err(|e| e.to_string())?;

    match instruction.instruction_type {
        InstructionType::Mov => assemble_mov(&mut assembler, instruction),
        InstructionType::Add => assemble_add(&mut assembler, instruction),
        InstructionType::Sub => assemble_sub(&mut assembler, instruction),
        InstructionType::And => assemble_and(&mut assembler, instruction),
        InstructionType::Or => assemble_or(&mut assembler, instruction),
        InstructionType::Xor => assemble_xor(&mut assembler, instruction),
        InstructionType::Inc => assemble_inc(&mut assembler, instruction),
        InstructionType::Dec => assemble_dec(&mut assembler, instruction),
        InstructionType::Neg => assemble_neg(&mut assembler, instruction),
        InstructionType::Not => assemble_not(&mut assembler, instruction),
        InstructionType::Shl => assemble_shl(&mut assembler, instruction),
        InstructionType::Shr => assemble_shr(&mut assembler, instruction),
        InstructionType::Rol => assemble_rol(&mut assembler, instruction),
        InstructionType::Ror => assemble_ror(&mut assembler, instruction),
        InstructionType::Push => assemble_push(&mut assembler, instruction),
        InstructionType::Pop => assemble_pop(&mut assembler, instruction),
        InstructionType::Cmp => assemble_cmp(&mut assembler, instruction),
        InstructionType::Test => assemble_test(&mut assembler, instruction),
        InstructionType::Jmp => assemble_jmp(&mut assembler, instruction),
        InstructionType::Je => assemble_je(&mut assembler, instruction),
        InstructionType::Jne => assemble_jne(&mut assembler, instruction),
        InstructionType::Jg => assemble_jg(&mut assembler, instruction),
        InstructionType::Jge => assemble_jge(&mut assembler, instruction),
        InstructionType::Jl => assemble_jl(&mut assembler, instruction),
        InstructionType::Jle => assemble_jle(&mut assembler, instruction),
        InstructionType::Call => assemble_call(&mut assembler, instruction),
        InstructionType::Ret => assemble_ret(&mut assembler, instruction),
        InstructionType::Paddd => assemble_paddd(&mut assembler, instruction), // Vector instruction
        // --- Assembly Wizardry Examples ---
        InstructionType::Bsf => assemble_bsf(&mut assembler, instruction),
        InstructionType::Cmovne => assemble_cmovne(&mut assembler, instruction),
        //_ => return Err(format!("Unsupported instruction: {:?}", instruction.instruction_type)),
    }?;

    assembler.assemble(0).map_err(|e| e.to_string())
}

fn assemble_mov(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if instruction.operands.len() != 2 {
        return Err("MOV instruction requires exactly two operands".to_string());
    }

    match (&instruction.operands[0], &instruction.operands[1]) {
        (Operand::Register(dest), Operand::Immediate(imm)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            assembler.mov(dest_reg, *imm as i64).map_err(|e| e.to_string())?;
        },
        (Operand::Register(dest), Operand::Register(src)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            let src_reg = parser_register_to_asm_register64(src);
            assembler.mov(dest_reg, src_reg).map_err(|e| e.to_string())?;
        },
        _ => return Err("Invalid operands for mov instruction".to_string()),
    }
    Ok(())
}

fn assemble_add(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    match (&instruction.operands[0], &instruction.operands[1]) {
        (Operand::Register(dest), Operand::Immediate(imm)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            assembler.add(dest_reg, *imm as i32).map_err(|e| e.to_string())?;
        },
        (Operand::Register(dest), Operand::Register(src)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            let src_reg = parser_register_to_asm_register64(src);
            assembler.add(dest_reg, src_reg).map_err(|e| e.to_string())?;
        },
        _ => return Err("Invalid operands for add instruction".to_string()),
    }
    Ok(())
}

fn assemble_sub(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    match (&instruction.operands[0], &instruction.operands[1]) {
        (Operand::Register(dest), Operand::Immediate(imm)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            assembler.sub(dest_reg, *imm as i32).map_err(|e| e.to_string())?;
        },
        (Operand::Register(dest), Operand::Register(src)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            let src_reg = parser_register_to_asm_register64(src);
            assembler.sub(dest_reg, src_reg).map_err(|e| e.to_string())?;
        },
        _ => return Err("Invalid operands for sub instruction".to_string()),
    }
    Ok(())
}

fn assemble_and(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    match (&instruction.operands[0], &instruction.operands[1]) {
        (Operand::Register(dest), Operand::Immediate(imm)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            assembler.and(dest_reg, *imm as i32).map_err(|e| e.to_string())?;
        },
        (Operand::Register(dest), Operand::Register(src)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            let src_reg = parser_register_to_asm_register64(src);
            assembler.and(dest_reg, src_reg).map_err(|e| e.to_string())?;
        },
        _ => return Err("Invalid operands for and instruction".to_string()),
    }
    Ok(())
}

fn assemble_or(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    match (&instruction.operands[0], &instruction.operands[1]) {
        (Operand::Register(dest), Operand::Immediate(imm)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            assembler.or(dest_reg, *imm as i32).map_err(|e| e.to_string())?;
        },
        (Operand::Register(dest), Operand::Register(src)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            let src_reg = parser_register_to_asm_register64(src);
            assembler.or(dest_reg, src_reg).map_err(|e| e.to_string())?;
        },
        _ => return Err("Invalid operands for or instruction".to_string()),
    }
    Ok(())
}

fn assemble_xor(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    match (&instruction.operands[0], &instruction.operands[1]) {
        (Operand::Register(dest), Operand::Immediate(imm)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            assembler.xor(dest_reg, *imm as i32).map_err(|e| e.to_string())?;
        },
        (Operand::Register(dest), Operand::Register(src)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            let src_reg = parser_register_to_asm_register64(src);
            assembler.xor(dest_reg, src_reg).map_err(|e| e.to_string())?;
        },
        _ => return Err("Invalid operands for xor instruction".to_string()),
    }
    Ok(())
}

fn assemble_inc(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Register(reg) = &instruction.operands[0] {
        let asm_reg = parser_register_to_asm_register64(reg);
        assembler.inc(asm_reg).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for inc instruction".to_string());
    }
    Ok(())
}

fn assemble_dec(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Register(reg) = &instruction.operands[0] {
        let asm_reg = parser_register_to_asm_register64(reg);
        assembler.dec(asm_reg).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for dec instruction".to_string());
    }
    Ok(())
}

fn assemble_neg(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Register(reg) = &instruction.operands[0] {
        let asm_reg = parser_register_to_asm_register64(reg);
        assembler.neg(asm_reg).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for neg instruction".to_string());
    }
    Ok(())
}

fn assemble_not(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Register(reg) = &instruction.operands[0] {
        let asm_reg = parser_register_to_asm_register64(reg);
        assembler.not(asm_reg).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for not instruction".to_string());
    }
    Ok(())
}

fn assemble_shl(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let (Operand::Register(reg), Operand::Immediate(shift)) = (&instruction.operands[0], &instruction.operands[1]) {
        let asm_reg = parser_register_to_asm_register64(reg);
        assembler.shl(asm_reg, *shift as i32).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operands for shl instruction".to_string());
    }
    Ok(())
}

fn assemble_shr(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let (Operand::Register(reg), Operand::Immediate(shift)) = (&instruction.operands[0], &instruction.operands[1]) {
        let asm_reg = parser_register_to_asm_register64(reg);
        assembler.shr(asm_reg, *shift as i32).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operands for shr instruction".to_string());
    }
    Ok(())
}

fn assemble_rol(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let (Operand::Register(reg), Operand::Immediate(shift)) = (&instruction.operands[0], &instruction.operands[1]) {
        let asm_reg = parser_register_to_asm_register64(reg);
        assembler.rol(asm_reg, *shift as i32).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operands for rol instruction".to_string());
    }
    Ok(())
}

fn assemble_ror(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let (Operand::Register(reg), Operand::Immediate(shift)) = (&instruction.operands[0], &instruction.operands[1]) {
        let asm_reg = parser_register_to_asm_register64(reg);
        assembler.ror(asm_reg, *shift as i32).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operands for ror instruction".to_string());
    }
    Ok(())
}

fn assemble_push(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Register(reg) = &instruction.operands[0] {
        let asm_reg = parser_register_to_asm_register64(reg);
        assembler.push(asm_reg).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for push instruction".to_string());
    }
    Ok(())
}

fn assemble_pop(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Register(reg) = &instruction.operands[0] {
        let asm_reg = parser_register_to_asm_register64(reg);
        assembler.pop(asm_reg).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for pop instruction".to_string());
    }
    Ok(())
}

fn assemble_cmp(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    match (&instruction.operands[0], &instruction.operands[1]) {
        (Operand::Register(dest), Operand::Immediate(imm)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            assembler.cmp(dest_reg, *imm as i32).map_err(|e| e.to_string())?;
        },
        (Operand::Register(dest), Operand::Register(src)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            let src_reg = parser_register_to_asm_register64(src);
            assembler.cmp(dest_reg, src_reg).map_err(|e| e.to_string())?;
        },
        _ => return Err("Invalid operands for cmp instruction".to_string()),
    }
    Ok(())
}

fn assemble_test(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    match (&instruction.operands[0], &instruction.operands[1]) {
        (Operand::Register(dest), Operand::Immediate(imm)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            assembler.test(dest_reg, *imm as i32).map_err(|e| e.to_string())?;
        },
        (Operand::Register(dest), Operand::Register(src)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            let src_reg = parser_register_to_asm_register64(src);
            assembler.test(dest_reg, src_reg).map_err(|e| e.to_string())?;
        },
        _ => return Err("Invalid operands for test instruction".to_string()),
    }
    Ok(())
}

fn assemble_jmp(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Immediate(target) = instruction.operands[0] {
        assembler.jmp(target as u64).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for jmp instruction".to_string());
    }
    Ok(())
}

fn assemble_je(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Immediate(target) = instruction.operands[0] {
        assembler.je(target as u64).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for je instruction".to_string());
    }
    Ok(())
}


fn assemble_jne(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Immediate(target) = instruction.operands[0] {
        assembler.jne(target as u64).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for jne instruction".to_string());
    }
    Ok(())
}

fn assemble_jg(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Immediate(target) = instruction.operands[0] {
        assembler.jg(target as u64).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for jg instruction".to_string());
    }
    Ok(())
}

fn assemble_jge(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Immediate(target) = instruction.operands[0] {
        assembler.jge(target as u64).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for jge instruction".to_string());
    }
    Ok(())
}

fn assemble_jl(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Immediate(target) = instruction.operands[0] {
        assembler.jl(target as u64).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for jl instruction".to_string());
    }
    Ok(())
}

fn assemble_jle(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Immediate(target) = instruction.operands[0] {
        assembler.jle(target as u64).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for jle instruction".to_string());
    }
    Ok(())
}

fn assemble_call(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if let Operand::Immediate(target) = instruction.operands[0] {
        assembler.call(target as u64).map_err(|e| e.to_string())?;
    } else {
        return Err("Invalid operand for call instruction".to_string());
    }
    Ok(())
}

fn assemble_ret(assembler: &mut CodeAssembler, _instruction: &Instruction) -> Result<(), String> {
    assembler.ret().map_err(|e| e.to_string())?;
    Ok(())
}

fn assemble_paddd(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if instruction.operands.len() != 2 {
        return Err("PADDD instruction requires exactly two operands".to_string());
    }

    match (&instruction.operands[0], &instruction.operands[1]) {
        (Operand::XmmRegister(dest), Operand::XmmRegister(src)) => {
            let dest_reg = xmm_index_to_register(*dest)
                .and_then(|r| xmm::get_xmm(r))
                .ok_or("Invalid destination XMM register")?;
            let src_reg = xmm_index_to_register(*src)
                .and_then(|r| xmm::get_xmm(r))
                .ok_or("Invalid source XMM register")?;
            assembler.paddd(dest_reg, src_reg).map_err(|e| e.to_string())?;
        }
        _ => return Err("Invalid operands for paddd instruction".to_string()),
    }
    Ok(())
}

fn xmm_index_to_register(index: u8) -> Option<Register> {
    match index {
        0 => Some(Register::XMM0),
        1 => Some(Register::XMM1),
        2 => Some(Register::XMM2),
        3 => Some(Register::XMM3),
        4 => Some(Register::XMM4),
        5 => Some(Register::XMM5),
        6 => Some(Register::XMM6),
        7 => Some(Register::XMM7),
        8 => Some(Register::XMM8),
        9 => Some(Register::XMM9),
        10 => Some(Register::XMM10),
        11 => Some(Register::XMM11),
        12 => Some(Register::XMM12),
        13 => Some(Register::XMM13),
        14 => Some(Register::XMM14),
        15 => Some(Register::XMM15),
        _ => None,
    }
}


// --- Advanced Assembly Instructions ---

fn assemble_bsf(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if instruction.operands.len() != 2 {
        return Err("BSF instruction requires exactly two operands".to_string());
    }

    match (&instruction.operands[0], &instruction.operands[1]) {
        (Operand::Register(dest), Operand::Register(src)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            let src_reg = parser_register_to_asm_register64(src);
            assembler.bsf(dest_reg, src_reg).map_err(|e| e.to_string())?;
        }
        _ => return Err("Invalid operands for bsf instruction".to_string()),
    }
    Ok(())
}

fn assemble_cmovne(assembler: &mut CodeAssembler, instruction: &Instruction) -> Result<(), String> {
    if instruction.operands.len() != 2 {
        return Err("CMOVNE instruction requires exactly two operands".to_string());
    }

    match (&instruction.operands[0], &instruction.operands[1]) {
        (Operand::Register(dest), Operand::Register(src)) => {
            let dest_reg = parser_register_to_asm_register64(dest);
            let src_reg = parser_register_to_asm_register64(src);
            assembler.cmovne(dest_reg, src_reg).map_err(|e| e.to_string())?;
        }
        _ => return Err("Invalid operands for cmovne instruction".to_string()),
    }
    Ok(())
}

fn parser_register_to_asm_register64(reg: &ParserRegister) -> AsmRegister64 {
    use iced_x86::code_asm::registers::*;
    match reg {
        ParserRegister::Rax => rax,
        ParserRegister::Rbx => rbx,
        ParserRegister::Rcx => rcx,
        ParserRegister::Rdx => rdx,
        ParserRegister::Rsi => rsi,
        ParserRegister::Rdi => rdi,
        ParserRegister::Rbp => rbp,
        ParserRegister::Rsp => rsp,
        ParserRegister::R8  => r8,
        ParserRegister::R9  => r9,
        ParserRegister::R10 => r10,
        ParserRegister::R11 => r11,
        ParserRegister::R12 => r12,
        ParserRegister::R13 => r13,
        ParserRegister::R14 => r14,
        ParserRegister::R15 => r15,
    }
}