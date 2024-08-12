use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, space0, space1}, // removed multispace0, alphanumeric1
    combinator::{map, map_res, opt}, // Removed value
    sequence::{delimited, preceded, tuple},
};

#[derive(Debug, PartialEq, Clone)]
pub enum Register {
    Rax, Rbx, Rcx, Rdx,
    Rsi, Rdi, Rbp, Rsp,
    R8, R9, R10, R11,
    R12, R13, R14, R15
}
#[derive(Debug, PartialEq, Clone)]
pub enum InstructionType {
    Mov, Add, Sub, And, Or, Xor,
    Inc, Dec, Neg, Not,
    Shl, Shr, Rol, Ror,
    Push, Pop,
    Cmp, Test,
    Jmp, Je, Jne, Jg, Jge, Jl, Jle,
    Call, Ret,
    Paddd, // Packed Add Doublewords
    Bsf,
    Cmovne,
    //TODO: Add other instructions over time
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Register(Register),
    Immediate(i32),
    XmmRegister(u8),
    // ... other operand types as needed
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegisterDisplayOptions {
    pub human_readable: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InputType {
    Instruction(Instruction),
    Register(Register, RegisterDisplayOptions),
    Memory(MemoryDumpOptions),  // Add options for register display
}


#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub operands: Vec<Operand>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemoryDumpOptions {
    pub address: u64,
    pub size: usize,
    pub format: MemoryDumpFormat,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MemoryDumpFormat {
    Hex,
    Decimal,
}

fn usize_decimal(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(input)
}

//╔═══════════════════════════════════════════════════════════════════╗ 
//║   ⇩ Memory Dump Command                                           ║  
//╚═══════════════════════════════════════════════════════════════════╝

fn memory_command(input: &str) -> IResult<&str, MemoryDumpOptions> {
    let (input, _) = tag("memory")(input)?;
    let (input, _) = space1(input)?; 

    let (input, address) = map_res(
        preceded(tag("0x"), hex_digit1), 
        |hex_str: &str| u64::from_str_radix(hex_str, 16)
    )(input)?;

    let (input, size) = opt(delimited(
        space1,
        preceded(alt((tag("-s"), tag("--size"))), delimited(space0, usize_decimal, space0)),
        space0
    ))(input)?;

    let (input, format) = opt(delimited(
        space1,
        alt((
            map(alt((tag("-x"), tag("--hex"))), |_| MemoryDumpFormat::Hex),
            map(alt((tag("-d"), tag("--decimal"))), |_| MemoryDumpFormat::Decimal),
        )),
        space0,
    ))(input)?;

    Ok((input, MemoryDumpOptions {
        address,
        size: size.unwrap_or(16),
        format: format.unwrap_or(MemoryDumpFormat::Hex),
    }))
}


// ╔═══════════════════════════════════════════════════════════════════╗ 
// ║   ⇩ Register Parsing Function                                     ║  
// ╚═══════════════════════════════════════════════════════════════════╝ 

fn register(input: &str) -> IResult<&str, Register> {
    alt((
        map_res(tag("rax"), |_| Ok::<Register, nom::error::Error<&str>>(Register::Rax)),
        map_res(tag("rbx"), |_| Ok::<Register, nom::error::Error<&str>>(Register::Rbx)),
        map_res(tag("rcx"), |_| Ok::<Register, nom::error::Error<&str>>(Register::Rcx)),
        map_res(tag("rdx"), |_| Ok::<Register, nom::error::Error<&str>>(Register::Rdx)),
        map_res(tag("rsi"), |_| Ok::<Register, nom::error::Error<&str>>(Register::Rsi)),
        map_res(tag("rdi"), |_| Ok::<Register, nom::error::Error<&str>>(Register::Rdi)),
        map_res(tag("rbp"), |_| Ok::<Register, nom::error::Error<&str>>(Register::Rbp)),
        map_res(tag("rsp"), |_| Ok::<Register, nom::error::Error<&str>>(Register::Rsp)),
        map_res(tag("r8"),  |_| Ok::<Register, nom::error::Error<&str>>(Register::R8)),
        map_res(tag("r9"),  |_| Ok::<Register, nom::error::Error<&str>>(Register::R9)),
        map_res(tag("r10"), |_| Ok::<Register, nom::error::Error<&str>>(Register::R10)),
        map_res(tag("r11"), |_| Ok::<Register, nom::error::Error<&str>>(Register::R11)),
        map_res(tag("r12"), |_| Ok::<Register, nom::error::Error<&str>>(Register::R12)),
        map_res(tag("r13"), |_| Ok::<Register, nom::error::Error<&str>>(Register::R13)),
        map_res(tag("r14"), |_| Ok::<Register, nom::error::Error<&str>>(Register::R14)),
        map_res(tag("r15"), |_| Ok::<Register, nom::error::Error<&str>>(Register::R15))
    ))(input)
}


//╔═══════════════════════════════════════════════════════════════════╗ 
//║   ⇩ Immediate Value Parser                                        ║  
//╚═══════════════════════════════════════════════════════════════════╝

fn immediate(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse())(input)
}

//╔═══════════════════════════════════════════════════════════════════╗ 
//║   ⇩ Input Parsers                                                 ║  
//╚═══════════════════════════════════════════════════════════════════╝

pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = space0(input)?; // Optional leading whitespace
    let (input, instruction_type) = parse_instruction_type(input)?;
    let (input, operands) = parse_operands(input)?;

    Ok((input, Instruction { instruction_type, operands }))
}

pub fn parse_input(input: &str) -> IResult<&str, InputType> {
    alt((
        map(parse_instruction, InputType::Instruction),
        map(
            tuple((register, opt(tag("-h")))), // Check for -h flag
            |(reg, human)| InputType::Register(
                reg,
                RegisterDisplayOptions { human_readable: human.is_some() },
            ),
        ),
        map(memory_command, |options| InputType::Memory(options))
    ))(input)
}


fn parse_operands(input: &str) -> IResult<&str, Vec<Operand>> {
    let (input, first_operand) = opt(delimited(space1, operand, space0))(input)?;
    let (input, second_operand) = opt(delimited(
        tuple((space0, opt(tag(",")), space0)),
        operand,
        space0
    ))(input)?;

    let operands = match (first_operand, second_operand) {
        (Some(op1), Some(op2)) => vec![op1, op2],
        (Some(op1), None) => vec![op1],
        _ => vec![],
    };

    Ok((input, operands))
}


fn parse_instruction_type(input: &str) -> IResult<&str, InstructionType> {
    alt((
        parse_arithmetic_instructions,
        parse_logic_instructions,
        parse_shift_rotate_instructions,
        parse_stack_instructions,
        parse_compare_instructions,
        parse_jump_instructions,
        parse_call_ret_instructions,
        parse_advanced_instructions,
    ))(input)
}

//╔═══════════════════════════════════════════════════════════════════╗ 
//║   ⇩ Instruction Parsers                                           ║  
//╚═══════════════════════════════════════════════════════════════════╝

fn parse_arithmetic_instructions(input: &str) -> IResult<&str, InstructionType> {
    alt((
        map(tag("mov"), |_| InstructionType::Mov),
        map(tag("add"), |_| InstructionType::Add),
        map(tag("sub"), |_| InstructionType::Sub),
        map(tag("inc"), |_| InstructionType::Inc),
        map(tag("dec"), |_| InstructionType::Dec),
        map(tag("neg"), |_| InstructionType::Neg),
    ))(input)
}

fn parse_logic_instructions(input: &str) -> IResult<&str, InstructionType> {
    alt((
        map(tag("and"), |_| InstructionType::And),
        map(tag("or"),  |_| InstructionType::Or),
        map(tag("xor"), |_| InstructionType::Xor),
        map(tag("not"), |_| InstructionType::Not),
    ))(input)
}

fn parse_shift_rotate_instructions(input: &str) -> IResult<&str, InstructionType> {
    alt((
        map(tag("shl"), |_| InstructionType::Shl),
        map(tag("shr"), |_| InstructionType::Shr),
        map(tag("rol"), |_| InstructionType::Rol),
        map(tag("ror"), |_| InstructionType::Ror),
    ))(input)
}

fn parse_stack_instructions(input: &str) -> IResult<&str, InstructionType> {
    alt((
        map(tag("push"), |_| InstructionType::Push),
        map(tag("pop"), |_| InstructionType::Pop),
    ))(input)
}

fn parse_compare_instructions(input: &str) -> IResult<&str, InstructionType> {
    alt((
        map(tag("cmp"), |_| InstructionType::Cmp),
        map(tag("test"), |_| InstructionType::Test),
    ))(input)
}

fn parse_jump_instructions(input: &str) -> IResult<&str, InstructionType> {
    alt((
        map(tag("jmp"), |_| InstructionType::Jmp),
        map(tag("je"), |_| InstructionType::Je),
        map(tag("jne"), |_| InstructionType::Jne),
        map(tag("jg"), |_| InstructionType::Jg),
        map(tag("jge"), |_| InstructionType::Jge),
        map(tag("jl"), |_| InstructionType::Jl),
        map(tag("jle"), |_| InstructionType::Jle),
    ))(input)
}

fn parse_call_ret_instructions(input: &str) -> IResult<&str, InstructionType> {
    alt((
        map(tag("call"), |_| InstructionType::Call),
        map(tag("ret"), |_| InstructionType::Ret),
    ))(input)
}

fn parse_advanced_instructions(input: &str) -> IResult<&str, InstructionType> {
    alt((
        map(tag("paddd"), |_| InstructionType::Paddd),
        map(tag("bsf"), |_| InstructionType::Bsf),
        map(tag("cmovne"), |_| InstructionType::Cmovne),
    ))(input)
}


//╔═══════════════════════════════════════════════════════════════════╗ 
//║   ⇩ Operand Parser                                                ║  
//╚═══════════════════════════════════════════════════════════════════╝

fn operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(register, Operand::Register),
        map(immediate, Operand::Immediate),
        map(xmm_register, Operand::XmmRegister),
    ))(input)
}

//╔═══════════════════════════════════════════════════════════════════╗ 
//║   ⇩ XMM Register                                                  ║  
//╚═══════════════════════════════════════════════════════════════════╝

fn xmm_register(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("xmm")(input)?;
    map_res(digit1, |s: &str| s.parse::<u8>())(input)
}
