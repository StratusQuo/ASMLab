use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use colored::*;

mod cpu;
mod parser;
mod assembler;

use cpu::CPU;
use parser::{parse_input, Instruction, InputType};
use assembler::assemble_instruction;

fn main() -> rustyline::Result<()> {
    let mut cpu = CPU::new();
    let mut rl = DefaultEditor::new()?;

    println!("{}", "Welcome to the Assembly REPL!".green().bold());
    println!("Type '{}' to quit, '{}' for detailed state, '{}' for compact state, or enter an assembly instruction.", 
             "exit".italic().yellow(), 
             "state".italic().yellow(),
             "cpu".italic().yellow());

    loop {
        let readline = rl.readline(">> ".cyan().bold().to_string().as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match line.trim() {
                    "exit" => break,
                    "state" => display_detailed_cpu_state(&cpu),
                    "cpu" => display_compact_cpu_state(&cpu),
                    input => {
                        match parse_input(input) {
                            Ok((_, InputType::Instruction(instruction))) => {
                                process_instruction(&instruction, &mut cpu);
                            }
                            Ok((_, InputType::Register(register, options))) => {
                                let formatted_value = cpu.format_register_value(&register, &options);
                                println!("{}", formatted_value);
                            }
                            Ok((_, InputType::Memory(options))) => {
                                cpu.dump_memory(&options);
                            }
                            Err(e) => println!("{} {}", "Error parsing input:".red(), e),
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    println!("{}", "Goodbye!".green());
    Ok(())
}

fn process_instruction(instruction: &Instruction, cpu: &mut CPU) {
    match assemble_instruction(instruction) {
        Ok(bytes) => {
            println!("{} {:?}", "Assembled bytes:".blue(), bytes);
            cpu.execute(instruction);
            println!("{}", "Instruction executed.".green());
        },
        Err(e) => println!("{} {}", " ERROR ".on_truecolor(188, 44, 26), e),
    }
}

fn visualize_register(name: &str, value: u64) {
    let bits = format!("{:064b}", value);
    let visualization = bits.chars()
        .map(|c| if c == '1' { "█".bright_green() } else { "▁".dimmed() })
        .collect::<Vec<_>>()
        .chunks(8)
        .map(|chunk| chunk.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(""))
        .collect::<Vec<_>>()
        .join(" ");
    
    println!("{:<4} {} {:#018x}", name, visualization, value);
}

fn visualize_xmm_register(name: &str, value: u128) {
    let bits = format!("{:0128b}", value);
    let visualization = bits.chars()
        .enumerate()
        .map(|(i, c)| {
            let color = match i / 32 {
                0 => Color::Red,
                1 => Color::Green,
                2 => Color::Blue,
                3 => Color::Yellow,
                _ => unreachable!(),
            };
            if c == '1' { "█".color(color) } else { "▁".color(color).dimmed() }
        })
        .collect::<Vec<_>>()
        .chunks(8)
        .map(|chunk| chunk.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(""))
        .collect::<Vec<_>>()
        .join(" ");
    
    println!("{:<5} {}", name, visualization);
}

fn display_compact_cpu_state(cpu: &CPU) {
    println!("{}", "CPU State:".yellow().bold());

    let registers = [
        ("rax", cpu.rax), ("r8", cpu.r8),
        ("rbx", cpu.rbx), ("r9", cpu.r9),
        ("rcx", cpu.rcx), ("r10", cpu.r10),
        ("rdx", cpu.rdx), ("r11", cpu.r11),
        ("rdi", cpu.rdi), ("r12", cpu.r12),
        ("rsi", cpu.rsi), ("r13", cpu.r13),
        ("rbp", cpu.rbp), ("r14", cpu.r14),
        ("rsp", cpu.rsp), ("r15", cpu.r15),
    ];

    for chunk in registers.chunks(2) {
        println!("{:<3} {:#018x}  {:<3} {:#018x}",
            chunk[0].0.cyan(),
            chunk[0].1,
            chunk[1].0.cyan(),
            chunk[1].1
        );
    }

    println!("\n{:<7} {:#018x}", "rip".cyan(), cpu.rip);
    println!("{:<7} {:#018x}", "rflags".cyan(), cpu.rflags);
    println!("{:<7} {:#018x}", "cs".cyan(), cpu.cs);
    println!("{:<7} {:#018x}", "fs".cyan(), cpu.fs);
    println!("{:<7} {:#018x}", "gs".cyan(), cpu.gs);

    println!("\n{}", "FLAGS:".yellow());
    let flags = [
        ("CF", cpu.cf), ("ZF", cpu.zf),
        ("SF", cpu.sf), ("OF", cpu.of),
    ];
    let active_flags: Vec<_> = flags.iter()
        .filter(|&&(_, value)| value)
        .map(|&(name, _)| name)
        .collect();
    println!("[{}]", active_flags.join(", "));
}

fn display_detailed_cpu_state(cpu: &CPU) {
    println!("{}", "Detailed CPU State:".yellow().bold());

    // Visualize general-purpose registers
    visualize_register("RAX", cpu.rax);
    visualize_register("RBX", cpu.rbx);
    visualize_register("RCX", cpu.rcx);
    visualize_register("RDX", cpu.rdx);
    visualize_register("RSI", cpu.rsi);
    visualize_register("RDI", cpu.rdi);
    visualize_register("RBP", cpu.rbp);
    visualize_register("RSP", cpu.rsp);
    visualize_register("R8", cpu.r8);
    visualize_register("R9", cpu.r9);
    visualize_register("R10", cpu.r10);
    visualize_register("R11", cpu.r11);
    visualize_register("R12", cpu.r12);
    visualize_register("R13", cpu.r13);
    visualize_register("R14", cpu.r14);
    visualize_register("R15", cpu.r15);

    println!("\n{:<7} {:#018x}", "RIP".cyan(), cpu.rip);

    // Visualize XMM registers
    println!("\nXMM Registers:");
    for (i, xmm_value) in cpu.xmm.iter().enumerate() {
        visualize_xmm_register(&format!("XMM{}", i), *xmm_value);
    }

    println!("\n{}", "FLAGS:".yellow());
    let flags = [
        ("CF", cpu.cf), ("ZF", cpu.zf),
        ("SF", cpu.sf), ("OF", cpu.of),
    ];
    let active_flags: Vec<_> = flags.iter()
        .filter(|&&(_, value)| value)
        .map(|&(name, _)| name.to_string())
        .collect();
    println!("[{}]", active_flags.join(", "));
}