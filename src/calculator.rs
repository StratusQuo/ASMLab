use std::f64;
use crate::cpu::CPU;

pub fn calculate(input: &str, cpu: &CPU) -> Result<String, String> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    
    if tokens.is_empty() {
        return Err("No input provided".to_string());
    }

    match tokens[0] {
        "hex" | "bin" | "dec" => convert_base(tokens),
        "and" | "or" | "xor" | "not" => bitwise_op(tokens),
        "sin" | "cos" | "tan" => trig_op(tokens),
        "+" | "-" | "*" | "/" => arithmetic_op(tokens),
        "shl" | "shr" => bit_shift(tokens),
        "rol" | "ror" => bit_rotate(tokens),
        "twos" => twos_complement(tokens),
        "float_to_ieee" => float_to_ieee754(tokens),
        "reg" => register_value(tokens, cpu),
        _ => Err("Unknown operation".to_string()),
    }
}

fn bit_shift(tokens: Vec<&str>) -> Result<String, String> {
    if tokens.len() != 3 {
        return Err("Usage: shl/shr <value> <shift amount>".to_string());
    }
    let value = tokens[1].parse::<u64>().map_err(|e| format!("Invalid value: {}", e))?;
    let shift = tokens[2].parse::<u32>().map_err(|e| format!("Invalid shift amount: {}", e))?;
    let result = match tokens[0] {
        "shl" => value << shift,
        "shr" => value >> shift,
        _ => unreachable!(),
    };
    Ok(format!("Result: {:#x} ({})", result, result))
}

fn bit_rotate(tokens: Vec<&str>) -> Result<String, String> {
    if tokens.len() != 3 {
        return Err("Usage: rol/ror <value> <rotate amount>".to_string());
    }
    let value = tokens[1].parse::<u64>().map_err(|e| format!("Invalid value: {}", e))?;
    let rotate = tokens[2].parse::<u32>().map_err(|e| format!("Invalid rotate amount: {}", e))?;
    let result = match tokens[0] {
        "rol" => value.rotate_left(rotate),
        "ror" => value.rotate_right(rotate),
        _ => unreachable!(),
    };
    Ok(format!("Result: {:#x} ({})", result, result))
}

fn twos_complement(tokens: Vec<&str>) -> Result<String, String> {
    if tokens.len() != 2 {
        return Err("Usage: twos <value>".to_string());
    }
    let value = tokens[1].parse::<i64>().map_err(|e| format!("Invalid value: {}", e))?;
    let result = (!value).wrapping_add(1);
    Ok(format!("Two's complement: {:#x} ({})", result, result))
}

fn float_to_ieee754(tokens: Vec<&str>) -> Result<String, String> {
    if tokens.len() != 2 {
        return Err("Usage: float_to_ieee <value>".to_string());
    }
    let value = tokens[1].parse::<f32>().map_err(|e| format!("Invalid float: {}", e))?;
    let bits = value.to_bits();
    Ok(format!("IEEE 754: {:#x}", bits))
}

fn register_value(tokens: Vec<&str>, cpu: &CPU) -> Result<String, String> {
    if tokens.len() != 2 {
        return Err("Usage: reg <register_name>".to_string());
    }
    let reg_name = tokens[1].to_uppercase();
    let value = match reg_name.as_str() {
        "RAX" => cpu.rax,
        "RBX" => cpu.rbx,
        "RCX" => cpu.rcx,
        "RDX" => cpu.rdx,
        // ... add other registers as needed
        _ => return Err(format!("Unknown register: {}", reg_name)),
    };
    Ok(format!("{} value: {:#x} ({})", reg_name, value, value))
}

fn convert_base(tokens: Vec<&str>) -> Result<String, String> {
    if tokens.len() != 2 {
        return Err("Usage: hex/bin/dec <value>".to_string());
    }
    
    let (base, value) = match tokens[0] {
        "hex" => (16, tokens[1]),
        "bin" => (2, tokens[1]),
        "dec" => (10, tokens[1]),
        _ => return Err("Invalid base specified".to_string()),
    };

    let value = i64::from_str_radix(value, base)
        .map_err(|e| format!("Invalid input: {}", e))?;
    
    Ok(format!("Hex: {:#x}\nDecimal: {}\nBinary: {:#b}", value, value, value))
}

fn bitwise_op(tokens: Vec<&str>) -> Result<String, String> {
    if tokens.len() < 2 {
        return Err("Not enough arguments for bitwise operation".to_string());
    }
    
    let op = tokens[0];
    let values: Result<Vec<u64>, _> = tokens[1..].iter().map(|&s| s.parse()).collect();
    let values = values.map_err(|e| format!("Invalid input: {}", e))?;
    
    let result = match op {
        "and" => values.iter().fold(u64::MAX, |acc, &x| acc & x),
        "or" => values.iter().fold(0, |acc, &x| acc | x),
        "xor" => values.iter().fold(0, |acc, &x| acc ^ x),
        "not" => !values[0],
        _ => return Err("Unknown bitwise operation".to_string()),
    };
    
    Ok(format!("Result: {:#x} ({})", result, result))
}

fn trig_op(tokens: Vec<&str>) -> Result<String, String> {
    if tokens.len() != 2 {
        return Err("Invalid number of arguments for trigonometric operation".to_string());
    }
    
    let op = tokens[0];
    let angle: f64 = tokens[1].parse().map_err(|e| format!("Invalid input: {}", e))?;
    
    let result = match op {
        "sin" => angle.to_radians().sin(),
        "cos" => angle.to_radians().cos(),
        "tan" => angle.to_radians().tan(),
        _ => return Err("Unknown trigonometric operation".to_string()),
    };
    
    Ok(format!("Result: {}", result))
}

fn arithmetic_op(tokens: Vec<&str>) -> Result<String, String> {
    if tokens.len() != 3 {
        return Err("Invalid number of arguments for arithmetic operation".to_string());
    }
    
    let op = tokens[0];
    let a: f64 = tokens[1].parse().map_err(|e| format!("Invalid first operand: {}", e))?;
    let b: f64 = tokens[2].parse().map_err(|e| format!("Invalid second operand: {}", e))?;
    
    let result = match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                return Err("Division by zero".to_string());
            }
            a / b
        },
        _ => return Err("Unknown arithmetic operation".to_string()),
    };
    
    Ok(format!("Result: {}", result))
}