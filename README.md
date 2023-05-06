# bed

bed is a toy programming language designed to run on a register-based virtual machine.
The name "bed" originates from "Binary EDitor," analogous to how "sed" comes from "Stream EDitor."

## Usage

This repository contains an interpreter for the bed language.
To interpret and execute the sample "Hello, World!" program (hello.bed), run the following command:

```console
cargo run -- bed/tests/hello.bed
```

### Example

The "Hello, World!" example (hello.bed) looks like this:

```
"Hello, World!"
qaig.q
laiwluo$a
```

Additional examples are available in the `bed/tests/` directory.

### Command-line Options

The bed interpreter supports the following command-line options:

| Option           | Description                                             |
| ---------------- | ------------------------------------------------------- |
| `-i`, `--input`  | Specify a file path to read from as the standard input. |
| `-o`, `--output` | Specify a file path to write to as the standard output. |

## Language Specification

### Virtual Machine

The bed language runs on a virtual machine with the following configurations:

#### Registers: Four 8-bit registers (D, A, B, C) and a 1-bit flag register (E)

- D (data) and A (accumulator) are computation registers.
- B (block) and C (cell) are addressing registers.
- E (error) indicates whether an error occurred during instruction execution.
- An 8-bit register can store a non-negative integer ranging from 0 to 255.
- A 1-bit register can store 0 or 1.
- At the beginning of the program execution, each register is initialized to 0.

#### Memory: 65536 bytes organized as 256 blocks of 256 bytes each

- The B register determines which block is being accessed.
- The C register determines the position within that block.
- At the beginning of the program execution, each memory cell is initialized to 0.

#### Additional Components: Macro Registry, Stream Map, and Input/Output Descriptors

- Macro Registry: Manages macros recorded during program execution.
- Stream Map: Associates streams (e.g., standard input/output, files, queues) with stream descriptors.
  - Contains 256 stream descriptors (0-255), each associated with at most one stream.
  - At the beginning of the program execution, the following streams are associated with their respective descriptors:
    - Standard input stream: descriptor 0
    - Standard output stream: descriptor 1
    - Standard error stream: descriptor 2
- Input/Output Descriptors: Specifies descriptors for input/output streams that are the target of stream operation instructions.
  - At the beginning of the program execution, the following descriptors are initialized to their respective numbers:
    - Input descriptor: 0
    - Output descriptor: 1

### Instruction Set

A source code of the BED language shall be interpreted as a sequence of bytes, with each byte basically corresponding to a specific instruction of the BED language.
The following list illustrates the correspondence between byte values and their respective BED language instructions.

| Byte Value                | Instruction                       |
| ------------------------- | --------------------------------- |
| 0x21 (`!`)                | Logical Negate                    |
| 0x22 (`"`)                | Quote                             |
| 0x23 (`#`)                | Comment                           |
| 0x24 (`$`)                | Repeat                            |
| 0x25 (`%`)                | Operate Stream                    |
| 0x26 (`&`)                | Bitwise AND                       |
| 0x27 (`'`)                | Direct                            |
| 0x28 (`(`)                | Left Bit Rotate                   |
| 0x29 (`)`)                | Right Bit Rotate                  |
| 0x2A (`*`)                | Multiply                          |
| 0x2B (`+`)                | Add                               |
| 0x2C (`,`)                | Getchar                           |
| 0x2D (`-`)                | Subtract                          |
| 0x2E (`.`)                | Putchar                           |
| 0x2F (`/`)                | Divide and Remainder              |
| 0x30 (`0`)                | Insert 0                          |
| 0x31 (`1`)                | Insert 1                          |
| 0x32 (`2`)                | Insert 2                          |
| 0x33 (`3`)                | Insert 3                          |
| 0x34 (`4`)                | Insert 4                          |
| 0x35 (`5`)                | Insert 5                          |
| 0x36 (`6`)                | Insert 6                          |
| 0x37 (`7`)                | Insert 7                          |
| 0x38 (`8`)                | Insert 8                          |
| 0x39 (`9`)                | Insert 9                          |
| 0x3A (`:`)                | Invoke Function                   |
| 0x3B (`;`)                | Define Function                   |
| 0x3C (`<`)                | Less than                         |
| 0x3D (`=`)                | Equal to                          |
| 0x3E (`>`)                | Greater than                      |
| 0x3F (`?`)                | Convert to Boolean                |
| 0x40 (`@`)                | Execute Macro                     |
| 0x41 (`A`) ... 0x5A (`Z`) | The same as the lowercase version |
| 0x5B (`[`)                | Increment                         |
| 0x5C (`\`)                | Check Flag                        |
| 0x5D (`]`)                | Decrement                         |
| 0x5E (`^`)                | Bitwise XOR                       |
| 0x5F (`_`)                | Clear Flag                        |
| 0x60 (`` ` ``)            | Evaluate Macro                    |
| 0x61 (`a`)                | Insert a                          |
| 0x62 (`b`)                | Insert b                          |
| 0x63 (`c`)                | Insert c                          |
| 0x64 (`d`)                | Insert d                          |
| 0x65 (`e`)                | Insert e                          |
| 0x66 (`f`)                | Insert f                          |
| 0x67 (`g`)                | Goto                              |
| 0x68 (`h`)                | Left                              |
| 0x69 (`i`)                | High                              |
| 0x6A (`j`)                | Down                              |
| 0x6B (`k`)                | Up                                |
| 0x6C (`l`)                | Right                             |
| 0x6D (`m`)                | Origin                            |
| 0x6E (`n`)                | Begin                             |
| 0x6F (`o`)                | Low                               |
| 0x70 (`p`)                | Swap                              |
| 0x71 (`q`)                | Record Macro                      |
| 0x72 (`r`)                | Load                              |
| 0x73 (`s`)                | Restore                           |
| 0x74 (`t`)                | Jump                              |
| 0x75 (`u`)                | Coordinate                        |
| 0x76 (`v`)                | Save                              |
| 0x77 (`w`)                | Store                             |
| 0x78 (`x`)                | Delete                            |
| 0x79 (`y`)                | Page                              |
| 0x7A (`z`)                | Zero                              |
| 0x7B (`{`)                | Left Bit Shift                    |
| 0x7C (`\|`)               | Bitwise OR                        |
| 0x7D (`}`)                | Right Bit Shift                   |
| 0x7E (`~`)                | Bitwise NOT                       |

- Byte values not listed in this table, including non-ASCII characters, are treated as No-Operation (Nop) Instructions.
- The `mod256` function is used in some descriptions below to ensure that results range from 0 to 255.
  - `mod256(x) = x % 256`

### Insert Instructions

#### Insert 0 (`0`)

- `A := mod256(A << 4) | 0;`

#### Insert 1 (`1`)

- `A := mod256(A << 4) | 1;`

#### Insert 2 (`2`)

- `A := mod256(A << 4) | 2;`

#### Insert 3 (`3`)

- `A := mod256(A << 4) | 3;`

#### Insert 4 (`4`)

- `A := mod256(A << 4) | 4;`

#### Insert 5 (`5`)

- `A := mod256(A << 4) | 5;`

#### Insert 6 (`6`)

- `A := mod256(A << 4) | 6;`

#### Insert 7 (`7`)

- `A := mod256(A << 4) | 7;`

#### Insert 8 (`8`)

- `A := mod256(A << 4) | 8;`

#### Insert 9 (`9`)

- `A := mod256(A << 4) | 9;`

#### Insert a (`a`)

- `A := mod256(A << 4) | 10;`

#### Insert b (`b`)

- `A := mod256(A << 4) | 11;`

#### Insert c (`c`)

- `A := mod256(A << 4) | 12;`

#### Insert d (`d`)

- `A := mod256(A << 4) | 13;`

#### Insert e (`e`)

- `A := mod256(A << 4) | 14;`

#### Insert f (`f`)

- `A := mod256(A << 4) | 15;`

### Data Instructions

#### High (`i`)

- `D := A;`

#### Low (`o`)

- `A := D;`

#### Swap (`p`)

- `x := A; A := D; D := x;`

#### Zero (`z`)

- `D := 0;`

#### Delete (`x`)

- `A := 0;`

### Move Instructions

#### Right (`l`)

- `C := mod256(C + 1);`

#### Left (`h`)

- `C := mod256(C - 1);`

#### Down (`j`)

- `C := mod256(C + 16);`

#### Up (`k`)

- `C := mod256(C - 16);`

#### Goto (`g`)

- `C := D;`

#### Jump (`t`)

- `B := D;`

#### Coordinate (`u`)

- `D := C;`

#### Page (`y`)

- `D := B;`

#### Origin (`m`)

- `C := 0;`

#### Begin (`n`)

- `B := 0;`

### Arithmetic Instructions

#### Add (`+`)

- `x := A + D; D := x >> 8; A := mod256(x);`

#### Subtract (`-`)

- `x := A - D; D := x < 0 ? mod256(-1) : 0; A := mod256(x);`

#### Multiply (`*`)

- `x := A * D; D := x >> 8; A := mod256(x);`

#### Divide and Remainder (`/`)

- If `D != 0`, `r := A % D; q := A / D; D := r; A := q;`.
- If `D == 0`, raise the error flag (`E := 1;`).

#### Increment (`[`)

- `A := mod256(A + 1);`

#### Decrement (`]`)

- `A := mod256(A - 1);`

### Bit Instructions

#### Left Bit Shift (`{`)

- `A := mod256(A << 1);`

#### Right Bit Shift (`}`)

- `A := mod256(A >> 1);`

#### Left Bit Rotate (`(`)

- `A := mod256(A << 1) | mod256(A >> 7);`

#### Right Bit Rotate (`)`)

- `A := mod256(A >> 1) | mod256(A << 7);`

#### Bitwise AND (`&`)

- `A := D & A;`

#### Bitwise OR (`|`)

- `A := D | A;`

#### Bitwise XOR (`^`)

- `A := D ^ A;`

#### Bitwise NOT (`~`)

- `A := ~A;`

### Comparison Instructions

#### Logical Negate (`!`)

- `A := A == 0 ? 1 : 0;`

#### Convert to Boolean (`?`)

- `A := A != 0 ? 1 : 0;`

#### Equal to (`=`)

- `A := D == A ? 1 : 0;`

#### Less than (`<`)

- `A := D < A ? 1 : 0;`

#### Greater than (`>`)

- `A := D > A ? 1 : 0;`

### Flag Instructions

#### Check Flag (`\`)

- `A := E;`

#### Clear Flag (`_`)

- `E := 0;`

### Memory Instruction

#### Load (`r`)

- `D := memory[B][C];`

#### Store (`w`)

- `memory[B][C] := D;`

#### Restore (`s`)

- `C := memory[B][D];`

#### Save (`v`)

- `memory[B][D] := C;`

#### Direct (`'`)

- Syntax: `'` _char_
  - _char_: any 1-byte character
- Semantics:
  - Write the value of _char_ as a byte to `memory[B][C]`.

#### Quote (`"`)

- Syntax: `"` _quote_ `"`
  - _quote_: a string of zero or more characters not including `"`
- Semantics:

  - Write the string _quote_ as a byte sequence to the memory in the range from `memory[B][C]` to `memory[B][255]`.
  - If the byte sequence _quote_ does not fit within the range, ignore the subsequent byte sequence and raise the error flag.
  - Update the value of the C register to point to the last position of the written byte sequence.

### Input/Output Instructions

#### Getchar (`,`)

- Read a byte from the input stream to `memory[B][C]`.
- Raise the error flag (`E := 1;`) without modifying memory, if an error occurs when reading from the input stream.

#### Putchar (`.`)

- Write a byte from `memory[B][C]` to the output stream.
- Raise the error flag (`E := 1;`), if an error occurs when writing to the output stream.

#### Operate Stream (`%`)

- Execute a variety of stream manipulation operations based on the value of the D register.

The following table illustrates the relationship between the value of the D register and their corresponding stream operations:

| D Register | Operation             |
| ---------- | --------------------- |
| 0          | GetDescriptor(input)  |
| 1          | GetDescriptor(output) |
| 2          | SetDescriptor(input)  |
| 3          | SetDescriptor(output) |
| 4          | Argc                  |
| 5          | Argv                  |
| 6          | OpenQueue             |
| 7          | OpenStandard          |
| 8          | OpenFile              |

Values of the D register other than those listed in the table are reserved.

##### GetDescriptor(input/output)

- Store the input/output descriptor number in the A register.

##### SetDescriptor(input/output)

- Assign the value of the A register to the input/output descriptor.

##### Argc

- Obtain the number of command line arguments and represent the value as a byte sequence in little-endian format, using the minimum length required for representation.
  - For example, use a 1-byte sequence for values between 0 and 255, and a 2-byte sequence for values between 256 and 65535.
- Write the byte sequence to the output stream.
  - Raise the error flag if an error occurs when writing to the output stream.
- Store the actual number of bytes written to the output stream in the A register.

##### Argv

- Read a sequence of M bytes, where M is the value of the A register, from the standard input and interpret it in little-endian format to obtain a nonnegative integer N.
  - Raise the error flag if unable to obtain N.
- Write the Nth command line argument string to the output stream as a byte string in UTF-8 format.
  - Raise the error flag if an error occurs when writing to the output stream .
- Raise the error flag if the Nth argument does not exist.

##### OpenQueue

- Create a new queue stream on the heap.
- Associate the new queue stream with the output descriptor in the Stream Map.
- If a stream has already been associated with the output descriptor, close the previous stream and replace it with the new queue stream.

##### OpenStandard

- Associate the standard stream with the output descriptor in the Stream Map according to the value of the A register:
  - `A == 0`: associate standard input.
  - `A == 1`: associate standard output.
  - `A == 2`: associate standard error.
  - `A == 255`: close associated stream (special case).
  - For other values of the A register, the operation is reserved.
- If a stream is already associated with the output descriptor, close the previous stream and replace it with the new standard stream.
- In the special case with `A == 255`, close the associated stream and leave the output descriptor with no associated stream.

##### OpenFile

- Obtain the file path by extracting all bytes from the input queue stream as indicated by the input descriptor and interpreting them as a UTF-8 string.
  - Raise the error flag if the input stream is not a queue stream.
- Open a file stream using the obtained file path and mode flags.
  - Use the bits in the A register to determine the mode flags for opening the file:
    - `A[0]`: read
    - `A[1]`: write
    - `A[2]`: append
    - `A[3]`: truncate
    - `A[4]`: create
    - `A[5]`: create_new
    - `A[i]` refers to the i-th least significant bit of the value of the A register.
  - Raise the error flag if the file opening fails.
- Associate the opened file stream with the output descriptor in the Stream Map.
- If a stream is already associated with the output descriptor, close the previous stream and replace it with the new file stream.

### Meta Instructions

#### Comment (`#`)

- Syntax: `#` _comment_ `\n`
  - _comment_: a string of zero or more characters, excluding newline character `\n`
- Semantics:
  - Ignore the instruction sequence _comment_.

#### Define Function (`;`)

- Syntax: `;` _name_ `\n` _body_ `;`
  - _name_: a string of zero or more characters, excluding newline character `\n`
  - _body_: a string of zero or more characters, excluding `;` at the beginning of the line
  - The first `;` and the last `;` shall be at the beginning of the line
- Semantics:
  - Define the instruction sequence _body_ as a function with the name _name_.
  - In a program, functions with the same name shall not be defined multiple times; the first definition in the source code has precedence.

#### Invoke Function (`:`)

- Syntax: `:` _name_ `\n`
  - _name_: a string of zero or more characters, excluding newline character `\n`
- Semantics:
  - Execute the function named _name_.
  - If the function named _name_ has not been defined, do nothing.

### Macro Instructions

#### Record Macro (`q`)

- Syntax: `q` _reg_ _macro_ `q`
  - _reg_: any 1-byte character
  - _macro_: a string of zero or more characters excluding `q`
    - _macro_ can contain a character `q` only within the following instructions: Direct, Quote, Comment, Invoke Function, Execute Macro, Repeat.
    - _macro_ shall not contain function definition.
- Semantics:
  - Register the instruction sequence _macro_ as a macro with the name _reg_ in the macro registry.
  - The previous macro with the name _reg_ in the macro registry is overwritten by the new macro with the same name.
  - Macro recording does not affect the current state of the VM, except for the macro registry.

#### Execute Macro (`@`)

- Syntax: `@` _reg_
  - _reg_: any 1-byte character
- Semantics:
  - Execute the macro named _reg_ from the macro registry.
  - If the macro named _reg_ does not exist in the macro registry, do nothing.

#### Repeat (`$`)

- Syntax: `$` _reg_
  - _reg_: any 1-byte character
- Semantics:
  - Execute the macro named _reg_ for the number of times specified by the value of the A register.
    - If the value of the A register is zero, do nothing.
  - Before each iteration of the macro execution, initialize the value of the A register to the number of iterations completed so far.
    - In other words, store 0 in the A register for the first iteration and store the original value minus 1 for the last iteration.
  - After all iterations have completed, restore the value of the A register to its original value.

#### Evaluate Macro (`` ` ``)

- Retrieve the macro with the name represented by the value of the D register from the macro registry.
- Execute the retrieved macro following the same procedure as the Execute Macro instruction.
