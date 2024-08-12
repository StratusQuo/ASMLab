use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use colored::*;

mod cpu;
mod parser;
mod assembler;
mod calculator;
mod script_mode;
mod syntax_highlighter;
mod user_functions;

use cpu::CPU;
use parser::{parse_input, parse_instruction, Instruction, InputType};
use assembler::assemble_instruction;
use calculator::calculate;
//use script_mode::execute_script;
use syntax_highlighter::highlight_syntax;
use script_mode::ScriptEnvironment;



#[derive(Debug, PartialEq)]
enum ReplMode {
    Single,
    Multi,
    Calculator,
    Script,
}

//╔═══════════════════════════════════════════════════════════════════╗ 
//║   ⇩ Main Loop                                                     ║  
//╚═══════════════════════════════════════════════════════════════════╝

fn main() -> rustyline::Result<()> {
    let mut cpu = CPU::new();
    let mut rl = DefaultEditor::new()?;
    let mut code_buffer: Vec<String> = Vec::new();
    let mut repl_mode = ReplMode::Single;
    let mut script_env = ScriptEnvironment::new();
    user_functions::load_user_functions(&mut script_env);

    println!("{}", "Welcome to the Enhanced Assembly REPL!".green().bold());
    print_help();

    loop {
        let prompt = match repl_mode {
            ReplMode::Single => ">> ".cyan().bold().to_string(),
            ReplMode::Multi => format!("{} ", " MULTI ".on_truecolor(188, 71, 73).truecolor(242, 232, 207).bold()),
            ReplMode::Calculator => format!("{} ", " CALC ".on_green().white().bold()),
            ReplMode::Script => format!("{} ", " SCRIPT ".on_magenta().white().bold()),
        };

        let readline = rl.readline(prompt.as_str());

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let highlighted_input = highlight_syntax(&line);
                println!("{}", highlighted_input);

                let trimmed = line.trim();
                match trimmed {
                    "exit" => break,
                    "help" => print_help(),
                    "cpu" => display_compact_cpu_state(&cpu),
                    "state" => display_detailed_cpu_state(&cpu),
                    ":single" => {
                        repl_mode = ReplMode::Single;
                        println!("Switched to single-instruction mode.");
                    }
                    ":multi" => {
                        repl_mode = ReplMode::Multi;
                        println!("Switched to multiple-instruction mode.");
                    }
                    ":calc" => {
                        repl_mode = ReplMode::Calculator;
                        println!("Switched to calculator mode.");
                    }
                    ":script" => {
                        repl_mode = ReplMode::Script;
                        println!("Switched to script mode.");
                    }
                    "run" => {
                        if repl_mode == ReplMode::Multi {
                            execute_multi_instructions(&mut cpu, &code_buffer);
                            code_buffer.clear();
                        } else {
                            println!("{} 'run' is only available in multi-instruction mode.", "ERROR:".red());
                        }
                    }
                    input => {
                        match repl_mode {
                            ReplMode::Single => handle_single_instruction(input, &mut cpu),
                            ReplMode::Multi => code_buffer.push(input.to_string()),
                            ReplMode::Calculator => {
                                match calculate(input, &cpu) {
                                    Ok(result) => println!("{}", result),
                                    Err(e) => println!("{} {}", "Calculation error:".red(), e),
                                }
                            }
                            ReplMode::Script => {
                                match script_env.execute_script(input, &cpu) {
                                    Ok(result) => println!("{}", result),
                                    Err(e) => println!("{} {}", "Script error:".red(), e),
                                }
                            }
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
        println!(); // Add extra newline for spacing
    }

    println!("{}", "Goodbye!".green());
    Ok(())
}

fn print_help() {
    println!("\n{}", "Available commands:".yellow().bold());
    println!("  {} - Exit the REPL", "exit".italic());
    println!("  {} - Display this help message", "help".italic());
    println!("  {} - Display compact CPU state", "cpu".italic());
    println!("  {} - Display detailed CPU state", "state".italic());
    println!("  {} - Switch to single-instruction mode", ":single".italic());
    println!("  {} - Switch to multiple-instruction mode", ":multi".italic());
    println!("  {} - Switch to calculator mode", ":calc".italic());
    println!("  {} - Switch to script mode", ":script".italic());
    println!("  {} - Execute instructions in multi-instruction mode", "run".italic());
    println!();
}

//╔═══════════════════════════════════════════════════════════════════╗ 
//║   ⇩ Instruction Processing                                        ║  
//╚═══════════════════════════════════════════════════════════════════╝

fn handle_single_instruction(input: &str, cpu: &mut CPU) {
    match parse_input(input) {
        Ok((_, InputType::Instruction(instruction))) => {
            process_instruction(&instruction, cpu);
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

fn execute_multi_instructions(cpu: &mut CPU, instructions: &[String]) {
    for (i, instruction_str) in instructions.iter().enumerate() {
        match parse_instruction(instruction_str) {
            Ok((_, instruction)) => {
                println!("Executing: {}", instruction_str);
                process_instruction(&instruction, cpu);
            }
            Err(e) => {
                println!("{} Error in instruction {}: {}", "ERROR:".red(), i + 1, e);
                return;
            }
        }
    }
    println!("{}", "All instructions executed successfully.".green());
}

fn process_instruction(instruction: &Instruction, cpu: &mut CPU) {
    match assemble_instruction(instruction) {
        Ok(bytes) => {
            println!("{} {:?}", "Assembled bytes:".blue(), bytes);
            cpu.execute(instruction);
            println!("{}", "Instruction executed.".green());
        },
        Err(e) => println!("{} {}", "ERROR:".red(), e),
    }
}

//╔═══════════════════════════════════════════════════════════════════╗ 
//║   ⇩ Register Visualization                                        ║  
//╚═══════════════════════════════════════════════════════════════════╝

fn visualize_register(name: &str, value: u64) {
    let bits = format!("{:064b}", value);
    let visualization = bits.chars()
        .enumerate()
        .map(|(i, c)| {
            let color = get_bit_color(i);
            if c == '1' {
                "█".color(color)
            } else {
                "▁".white()
            }
        })
        .enumerate()
        .map(|(i, colored_char)| {
            if (i + 1) % 8 == 0 && i < 63 {
                format!("{} ", colored_char)
            } else {
                colored_char.to_string()
            }
        })
        .collect::<String>();

    println!("{:<4} {} {:#018x}", name.white(), visualization, value);
}

fn get_bit_color(index: usize) -> Color {
    const COLOR_RANGES: [[(u8, u8, u8); 8]; 8] = [
        // Burg
        [(255,235,238), (255,205,210), (239,154,154), (229,115,115), (239,83,80), (244,67,54), (211,47,47), (183,28,28)],
        // BurgYl
        [(255,248,225), (255,236,179), (255,224,130), (255,202,40), (255,167,38), (251,140,0), (245,124,0), (230,81,0)],
        // RedOr
        [(255,243,224), (255,224,178), (255,204,128), (255,183,77), (255,152,0), (251,140,0), (245,124,0), (230,81,0)],
        // OrYel
        [(255,253,231), (255,249,196), (255,245,157), (255,241,118), (255,238,88), (255,235,59), (253,216,53), (251,192,45)],
        // Peach
        [(255,248,225), (255,236,179), (255,224,130), (255,213,79), (255,202,40), (255,193,7), (255,179,0), (255,160,0)],
        // PinkYl
        [(252,228,236), (255,205,210), (255,171,145), (255,138,101), (255,112,67), (255,87,34), (244,81,30), (230,74,25)],
        // Mint
        [(224,242,241), (178,223,219), (128,203,196), (77,182,172), (38,166,154), (0,150,136), (0,137,123), (0,121,107)],
        // BluGrn
        [(224,242,241), (178,223,219), (128,203,196), (77,182,172), (38,166,154), (0,150,136), (0,137,123), (0,121,107)],
    ];

    let octal = index / 8;
    let position_in_octal = index % 8;

    let (r, g, b) = COLOR_RANGES[octal][position_in_octal];
    Color::TrueColor { r, g, b }
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