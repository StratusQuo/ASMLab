use crate::cpu::CPU;
use std::collections::HashMap;

type ScriptFunction = fn(&[&str], &CPU, &mut HashMap<String, u64>) -> Result<String, String>;

pub struct ScriptEnvironment {
    functions: HashMap<String, ScriptFunction>,
    variables: HashMap<String, u64>,
}

impl ScriptEnvironment {
    pub fn new() -> Self {
        let mut env = ScriptEnvironment {
            functions: HashMap::new(),
            variables: HashMap::new(),
        };
        env.load_default_functions();
        env
    }

    fn load_default_functions(&mut self) {
        self.add_function("decimal", decimal);
        self.add_function("→", assignment);
        self.add_function("+", |args, cpu, vars| arithmetic(args, cpu, vars, '+')); 
        self.add_function("-", |args, cpu, vars| arithmetic(args, cpu, vars, '-'));
        self.add_function("×", |args, cpu, vars| arithmetic(args, cpu, vars, '*'));
        self.add_function("÷", |args, cpu, vars| arithmetic(args, cpu, vars, '/'));

        //APL Operations:
        // New operations
        self.add_function("∧", |args, cpu, vars| bitwise(args, cpu, vars, '&'));
        self.add_function("∨", |args, cpu, vars| bitwise(args, cpu, vars, '|'));
        self.add_function("⊻", |args, cpu, vars| bitwise(args, cpu, vars, '^'));
        self.add_function("⌽", rotate);
        self.add_function("↑", |args, cpu, vars| shift(args, cpu, vars, true));
        self.add_function("↓", |args, cpu, vars| shift(args, cpu, vars, false));
        self.add_function("?", memory_operation);
        self.add_function("ι", range);
    }

    pub fn add_function(&mut self, name: &str, func: ScriptFunction) {
        self.functions.insert(name.to_string(), func);
    }

    pub fn execute_script(&mut self, script: &str, cpu: &CPU) -> Result<String, String> {
        let lines: Vec<&str> = script.lines().collect();
        let mut output = String::new();

        for line in lines {
            let result = self.execute_line(line.trim(), cpu)?;
            output.push_str(&result);
            output.push('\n');
        }

        Ok(output)
    }

    fn execute_line(&mut self, line: &str, cpu: &CPU) -> Result<String, String> {
        if line.is_empty() || line.starts_with("//") {
            return Ok(String::new());
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            return Ok(String::new());
        }

        let function_name = tokens[0];
        let args = &tokens[1..];

        match self.functions.get(function_name) {
            Some(func) => func(args, cpu, &mut self.variables),
            None => Err(format!("Unknown function: {}", function_name)),
        }
    }
}

fn decimal(args: &[&str], cpu: &CPU, _vars: &mut HashMap<String, u64>) -> Result<String, String> {
    if args.len() != 1 {
        return Err("Usage: decimal <register>".to_string());
    }
    let register = args[0].to_lowercase();
    match register.as_str() {
        "rax" => Ok(format!("RAX in decimal: {}", cpu.rax)),
        "rbx" => Ok(format!("RBX in decimal: {}", cpu.rbx)),
        "rcx" => Ok(format!("RCX in decimal: {}", cpu.rcx)),
        "rdx" => Ok(format!("RDX in decimal: {}", cpu.rdx)),
        "rdi" => Ok(format!("RDI in decimal: {}", cpu.rdi)),
        "rsi" => Ok(format!("RSI in decimal: {}", cpu.rsi)),
        "rbp" => Ok(format!("RBP in decimal: {}", cpu.rbp)),
        "rsp" => Ok(format!("RSP in decimal: {}", cpu.rsp)),
        "r8" => Ok(format!("R8 in decimal: {}", cpu.r8)),
        "r9" => Ok(format!("R9 in decimal: {}", cpu.r9)),
        "r10" => Ok(format!("R10 in decimal: {}", cpu.r10)),
        "r11" => Ok(format!("R11 in decimal: {}", cpu.r11)),
        "r12" => Ok(format!("R12 in decimal: {}", cpu.r12)),
        "r13" => Ok(format!("R13 in decimal: {}", cpu.r13)),
        "r14" => Ok(format!("R14 in decimal: {}", cpu.r14)),
        "r15" => Ok(format!("R15 in decimal: {}", cpu.r15)),
        _ => Err(format!("Unknown register: {}", register)),
    }
}

fn assignment(args: &[&str], _cpu: &CPU, vars: &mut HashMap<String, u64>) -> Result<String, String> {
    if args.len() != 2 {
        return Err("Invalid assignment syntax".to_string());
    }
    let value = evaluate_expression(args[1], vars)?;
    vars.insert(args[0].to_string(), value);
    Ok(format!("{} ← {}", args[0], value))
}

fn arithmetic(args: &[&str], _cpu: &CPU, vars: &mut HashMap<String, u64>, op: char) -> Result<String, String> {
    if args.len() != 2 {
        return Err("Invalid arithmetic syntax".to_string());
    }
    let a = get_value(args[0], vars)?;
    let b = get_value(args[1], vars)?;
    let result = match op {
        '+' => a.wrapping_add(b),
        '-' => a.wrapping_sub(b),
        '*' => a.wrapping_mul(b),
        '/' => if b == 0 { return Err("Division by zero".to_string()); } else { a / b },
        _ => return Err("Unknown arithmetic operation".to_string()),
    };
    Ok(format!("Result: {}", result))
}

fn bitwise(args: &[&str], _cpu: &CPU, vars: &mut HashMap<String, u64>, op: char) -> Result<String, String> {
    if args.len() != 2 {
        return Err("Invalid bitwise syntax".to_string());
    }
    let a = get_value(args[0], vars)?;
    let b = get_value(args[1], vars)?;
    let result = match op {
        '&' => a & b,
        '|' => a | b,
        '^' => a ^ b,
        _ => return Err("Unknown bitwise operation".to_string()),
    };
    Ok(format!("Result: {:#x}", result))
}

fn rotate(args: &[&str], _cpu: &CPU, vars: &mut HashMap<String, u64>) -> Result<String, String> {
    if args.len() != 2 {
        return Err("Invalid rotate syntax".to_string());
    }
    let value = get_value(args[0], vars)?;
    let shift: u32 = args[1].parse().map_err(|_| "Invalid shift amount".to_string())?;
    let result = value.rotate_left(shift);
    Ok(format!("Result: {:#x}", result))
}

fn shift(args: &[&str], _cpu: &CPU, vars: &mut HashMap<String, u64>, left: bool) -> Result<String, String> {
    if args.len() != 2 {
        return Err("Invalid shift syntax".to_string());
    }
    let value = get_value(args[0], vars)?;
    let shift: u32 = args[1].parse().map_err(|_| "Invalid shift amount".to_string())?;
    let result = if left { value << shift } else { value >> shift };
    Ok(format!("Result: {:#x}", result))
}

fn memory_operation(args: &[&str], cpu: &CPU, _vars: &mut HashMap<String, u64>) -> Result<String, String> {
    if args.len() != 1 {
        return Err("Invalid memory operation syntax".to_string());
    }
    let address: usize = args[0].parse().map_err(|_| "Invalid memory address".to_string())?;
    if address >= cpu.memory.len() {
        return Err("Memory address out of bounds".to_string());
    }
    Ok(format!("Value at address {:#x}: {:#x}", address, cpu.memory[address]))
}

fn range(args: &[&str], _cpu: &CPU, _vars: &mut HashMap<String, u64>) -> Result<String, String> {
    if args.len() != 1 && args.len() != 2 {
        return Err("Invalid range syntax".to_string());
    }
    let start = if args.len() == 2 {
        args[0].parse().map_err(|_| "Invalid start value".to_string())?
    } else {
        0
    };
    let end: u64 = args[args.len() - 1].parse().map_err(|_| "Invalid end value".to_string())?;
    let range: Vec<u64> = (start..end).collect();
    Ok(format!("Range: {:?}", range))
}


fn get_value(token: &str, vars: &HashMap<String, u64>) -> Result<u64, String> {
    if let Ok(value) = token.parse::<u64>() {
        Ok(value)
    } else if let Some(value) = vars.get(token) {
        Ok(*value)
    } else {
        Err(format!("Unknown variable or invalid literal: {}", token))
    }
}

fn evaluate_expression(expr: &str, vars: &HashMap<String, u64>) -> Result<u64, String> {
    // Implement expression evaluation here
    // For now, just return the value directly
    get_value(expr, vars)
}