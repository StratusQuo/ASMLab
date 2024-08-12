# ASMLab - An Interactive x86 Assembly Language REPL

***Practice x86 Assembly, One Instruction at a Time***

<img width="1289" alt="image" src="https://github.com/user-attachments/assets/e2351c93-ecff-4596-8fbc-61ef852c5bbb">


## Intro
`asmlab` is a powerful and interactive command-line tool that lets you test out assembly commands in a simulated environment. 

This project is a reimagining of the excellent [`asmrepl`](https://github.com/tenderlove/asmrepl), but ported to Rust with the following features :

- **Simulated CPU:** A virtual x86 environment where you can execute instructions and observe their effects on registers, flags, and memory.
- **Multiple Modes:**  Seamlessly switch between modes to enhance your assembly experience:
  - **Single-Instruction Mode:** Execute and analyze one instruction at a time.
  - **Multi-Instruction Mode:** Write and run small assembly programs.
  - **Calculator Mode:** Perform arithmetic, bitwise, and trigonometric calculations, along with base conversions.
  - **Script Mode:**  Define variables, use functions, and write scripts with an APL-inspired syntax.
- **Syntax Highlighting:** Code input is highlighted for improved readability.
- **Comprehensive Instruction Set:** Supports a wide range of x86 instructions, covering common operations and a few advanced instructions as well.
- **Visual Register Representation:** See the binary representation of registers for a deeper understanding of bit-level operations.



## Installation

1. **Clone the Repository:**

   ```bash
   git clone git@github.com:StratusQuo/assemblyrepl.git
   ```

2. **Build the Project:**

   ```bash
   cd assemblyrepl
   cargo build
   ```

3.  **After building with Cargo**:

   Run the app with `cargo run` or copy the `asmlab` executable from the `target/release` directory to a directory in your `PATH`:

   ```bash      
   cp target/release/asmlab /usr/local/bin/
   ```

   

## Usage

1. **Run the REPL:**

   ```bash
   cargo run
   ```

2. **Explore!** Use the following commands and features:

   - **`exit`:** Quit the REPL.
   - **`help`:** Display the help message (a list of available commands).
   - **`cpu`:** Show a compact view of the CPU state, including register values and flags.
   - **`state`:**  Display a detailed view of the CPU state, with register values visualized in binary.

   **Single-Instruction Mode (Default):**

   <img width="749" alt="image" src="https://github.com/user-attachments/assets/6713ae82-d191-4ec6-b6ae-51e5fee56734">


   - Enter a single assembly instruction (e.g., `mov rax, 5`) and press Enter to assemble and execute it.
   - Type a register name (e.g., `rax`) to see its value. 
   - Use the `memory` command to inspect memory:
	 - `memory 0x100`: Dumps 16 bytes in hexadecimal starting at address `0x100`.
	 - `memory 0x100 -s 32`: Dumps 32 bytes starting at address `0x100`.
	 - `memory 0x100 -d`:  Dumps 16 bytes in decimal starting at `0x100`.

   **Multi-Instruction Mode:**
   
   Type **`:multi`** to enter _multi-line_ mode:
   
   <img width="1356" alt="image" src="https://github.com/user-attachments/assets/2490c3a9-c3d9-4f97-8d6f-ab1ddb960f16">
   
   - Enter your assembly instructions _(one instruction per line)_
   - An empty line indicates the end of your code block.
   - Type `run` to assemble and execute the code you've entered.

   **Calculator Mode:**

   Type **`:calc`** to enter calculator mode:

   <img width="897" alt="image" src="https://github.com/user-attachments/assets/b250900a-c84c-412a-8758-6be11e95f4fd">

   In calc mode you can perform the following calculations and conversions using the below commands:
	 - **`hex <value>`:** Convert a hexadecimal value to decimal and binary.
	 - **`bin <value>`:** Convert a binary value to decimal and hexadecimal.
	 - **`dec <value>`:** Convert a decimal value to hexadecimal and binary.
	 - **`and <value1> <value2> ...`:**  Perform a bitwise AND operation on the given values.
	 - **`or <value1> <value2> ...`:**  Perform a bitwise OR operation on the given values.
	 - **`xor <value1> <value2> ...`:**  Perform a bitwise XOR operation on the given values.
	 - **`not <value>`:** Perform a bitwise NOT operation on the value.
	 - **`sin <angle>`:** Calculate the sine of the angle (in degrees).
	 - **`cos <angle>`:** Calculate the cosine of the angle (in degrees).
	 - **`tan <angle>`:** Calculate the tangent of the angle (in degrees).
	 - **`<value1> + <value2>`:** Add two values.
	 - **`<value1> - <value2>`:** Subtract two values.
	 - **`<value1> * <value2>`:** Multiply two values.
	 - **`<value1> / <value2>`:** Divide two values.
	 - **`shl <value> <amount>`:** Shift the bits of the value left by the specified amount.
	 - **`shr <value> <amount>`:** Shift the bits of the value right by the specified amount.
	 - **`rol <value> <amount>`:** Rotate the bits of the value left by the specified amount.
	 - **`ror <value> <amount>`:** Rotate the bits of the value right by the specified amount.
	 - **`twos <value>`:** Calculate the two's complement of a value.
	 - **`float_to_ieee <value>`:** Convert a floating-point number to its IEEE 754 representation. 

   **Script Mode:**

   Type `:script` to enter script mode:

   <img width="591" alt="image" src="https://github.com/user-attachments/assets/c2942af9-0352-4872-bf48-4e192e26f286">


   _**Note:** There's still a lot of work to be done here -- there should be some fixes coming soon to make this mode more useful._

   - Define variables and write multi-line scripts that can use the following arithmetic & APL operators:
	 - **`decimal <register>`:** Display the decimal value of a register.
	 - **`<variable> → <value>`:** Assign a value to a variable.
	 - **`<value1> + <value2>`:** Add two values. 
	 - **`<value1> - <value2>`:** Subtract two values. 
	 - **`<value1> × <value2>`:** Multiply two values. 
	 - **`<value1> ÷ <value2>`:** Divide two values. 
	 - **`<value1> ∧ <value2>`:** Bitwise AND.
	 - **`<value1> ∨ <value2>`:** Bitwise OR.
	 - **`<value1> ⊻ <value2>`:** Bitwise XOR.
	 - **`⌽ <value> <amount>`:** Rotate the bits of the value left by the specified amount. 
	 - **`↑ <value> <amount>`:** Shift the bits of the value left by the specified amount.
	 - **`↓ <value> <amount>`:** Shift the bits of the value right by the specified amount.
	 - **`? <address>`:** Get the value at a memory address. 
	 - **`ι <end>`:** Create a range from 0 to `<end>`.
	 - **`ι <start> <end>`:** Create a range from `<start>` to `<end>`.



## Examples 

**Single Instruction Mode:**

```
>> mov rax, 10
>> add rax, 5
>> rax
Rax: 0x000000000000000f
```

**Multi-Instruction Mode:**

```
>> :multi
.. mov rax, 1
.. mov rbx, 2
.. add rax, rbx
.. run
Executing: mov rax, 1
Assembled bytes: [48, c7, c0, 01, 00, 00, 00]
Instruction executed.
Executing: mov rbx, 2
Assembled bytes: [48, c7, c3, 02, 00, 00, 00]
Instruction executed.
Executing: add rax, rbx
Assembled bytes: [48, 01, d8]
Instruction executed.
All instructions executed successfully.
>> rax
Rax: 0x0000000000000003
```

**Calculator Mode:**

```
>> :calc
>> hex ff
Hex: 0xff
Decimal: 255
Binary: 0b11111111
>> 10 + 20
Result: 30
>> sin 45
Result: 0.7071067811865475 
```

**Script Mode:**

```
>> :script
.. count → 10
count ← 10
.. count + 5
Result: 15
.. decimal rax
RAX in decimal: 3
```



## Features

- **Comprehensive Instruction Set:**  Supports a wide range of x86 instructions, including arithmetic, logic, bit manipulation, control flow, stack operations, and more. 
- **Interactive REPL:** Provides a user-friendly interface for experimenting with assembly and getting immediate feedback.
- **Simulated CPU:**  Offers a safe environment to learn and practice assembly without risk to your system.
- **Multiple Execution Modes:** 
  - **Single-Instruction Mode:** Step through instructions one by one for debugging and analysis.
  - **Multi-Instruction Mode:** Write and execute short assembly programs.
  - **Calculator Mode:** Perform calculations and conversions related to binary, hexadecimal, and decimal numbers, as well as trigonometric and bitwise operations.
  - **Script Mode:** Define variables and write more complex scripts using a custom syntax inspired by APL.
- **Syntax Highlighting:** Assembly instructions are highlighted for improved readability.
- **Visual Register Representation:**  Visualize register values as binary strings, making it easier to understand bitwise operations and flag manipulation.



## Contributing

Contributions are welcome! If you find bugs, have feature suggestions, or want to contribute to the codebase, please open an issue or submit a pull request on GitHub.



## License

This project is licensed under the MIT License.
