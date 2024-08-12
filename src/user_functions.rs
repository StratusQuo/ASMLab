use crate::script_mode::ScriptEnvironment;
use std::collections::HashMap;

pub fn load_user_functions(env: &mut ScriptEnvironment) {
    env.add_function("double", double);
    env.add_function("binary", binary);
}

fn double(args: &[&str], _cpu: &crate::cpu::CPU, _vars: &mut HashMap<String, u64>) -> Result<String, String> {
    if args.len() != 1 {
        return Err("Usage: double <value>".to_string());
    }
    let value: u64 = args[0].parse().map_err(|_| "Invalid number".to_string())?;
    Ok(format!("Result: {}", value * 2))
}

fn binary(args: &[&str], cpu: &crate::cpu::CPU, _vars: &mut HashMap<String, u64>) -> Result<String, String> {
    if args.len() != 1 {
        return Err("Usage: binary <register>".to_string());
    }
    let register = args[0].to_lowercase();
    let value = match register.as_str() {
        "rax" => cpu.rax,
        "rbx" => cpu.rbx,
        // Add other registers...
        _ => return Err(format!("Unknown register: {}", register)),
    };
    Ok(format!("{} in binary: {:b}", register.to_uppercase(), value))
}