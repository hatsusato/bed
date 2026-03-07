# bed (Binary EDitor)

bed is an esoteric programming language (Esolang) and its execution environment, heavily inspired by Vim's keybindings and operational philosophy.
It features an unorthodox and mind-bending Instruction Set Architecture (ISA) that allows you to navigate a 2D binary space like a text editor, select ranges (yank), perform calculations, and traverse Z-axis stack frames.

# 🌌 Architecture

The bed virtual machine has the following architecture:

- Main Memory: A 16 × 16 byte (256 bytes) 2D torus space. The top, bottom, left, and right edges wrap around.
- Z-Stack: The main memory is stacked along the Z-axis, consisting of 256 layers (frames) from 0 to 255.
- Pointer (P): The current X coordinate (4 bits) and Y coordinate (4 bits).
- Accumulator (A): The primary 1-byte (8-bit) register for arithmetic and logic operations.
- Secondary (S): A 1-byte register used to store the byproducts of operations (e.g., carry, remainder, upper byte of multiplication) or for temporary data preservation.
- Yank Buffer: A clipboard that holds data from ranges selected in Visual mode.

# 🚀 Modes

The runtime operates in two modes:

- Interactive Mode (TUI): Launched via the `bed` command without arguments. It acts as a binary editor, allowing you to code and execute while visualizing the memory space and registers in real-time.
- Batch Mode: Launched via `bed script.bed`. It runs headlessly without rendering the UI, acting as a pure, high-speed interpreter utilizing only standard I/O.

# ⌨️ Instruction Set Architecture (ISA)

1. Prefixes

| Key                       | Instruction    |
| ------------------------- | -------------- |
| `0`-`9`, `a`-`f`, `A`-`F` | Hex Multiplier |

2. Motion & Jump

| Key             | Instruction   |
| --------------- | ------------- |
| `h` `j` `k` `l` | Relative Move |
| `H` `J` `K` `L` | Edge Move     |
| `G`             | Dereference   |
| `'`             | Jump to A     |
| `` ` ``         | Jump to VS    |

3. Search

| Key       | Instruction |
| --------- | ----------- |
| `n` / `N` | Find Value  |
| `w` / `W` | Find Word   |
| `z` / `Z` | Find Zero   |

4. Registers & Memory

| Key       | Instruction         |
| --------- | ------------------- |
| `x` / `X` | Zero A / Zero M     |
| `_`       | Zero Frame          |
| `s` / `S` | Swap M / Swap S     |
| `m` / `M` | Get P / Set P       |
| `r`       | Replace / Fill      |
| `t` / `T` | Target Read / Write |

5. Arithmetic & Logic

| Key              | Instruction            |
| ---------------- | ---------------------- |
| `+` `-` `*` `/`  | Add / Sub / Mul / Div  |
| `&` `\|` `^` `~` | AND / OR / XOR / NOT   |
| `<` `>` `=`      | Compare                |
| `,` `.`          | Dec / Inc              |
| `!` `?`          | Logical NOT / Binarize |
| `(` `)`          | Shift Left / Right     |
| `{` `}`          | Rotate Left / Right    |

6. Visual Mode & Clipboard

| Key       | Instruction             |
| --------- | ----------------------- |
| `v` / `V` | Visual Mode             |
| `g`       | Cancel                  |
| `y` / `Y` | Yank / Cut              |
| `p` / `P` | Paste / Paste at Origin |
| `u` / `U` | Unpack / Pack           |

7. Z-Stack & I/O

| Key       | Instruction       |
| --------- | ----------------- |
| `[` / `]` | Push / Pop        |
| `i` / `I` | In Point / Range  |
| `o` / `O` | Out Point / Range |
| `$`       | Redirect          |

8. Macro & Control Flow

| Key       | Instruction        |
| --------- | ------------------ |
| `%`       | Dynamic Multiplier |
| `q` / `@` | Record / Call      |
| `R`       | Return             |
| `"`       | String             |
| `\`       | Escape ASCII       |
| `#`       | Comment            |
| `Q`       | Query Depth        |
| `;`       | Exit               |
| `:`       | Command Mode       |

# 📖 Instruction Reference

## Prefixes

### Hex Multiplier — `0`-`9` `a`-`f` `A`-`F`

Repeats the immediately following instruction the specified number of times in hexadecimal.

## Motion & Jump

### Relative Move — `h` `j` `k` `l`

Move left, down, up, right. Wraps around the torus edges.

### Edge Move — `H` `J` `K` `L`

Absolute jump to the edges: X=0, Y=15, Y=0, X=15 respectively.

### Deref — `G`

Warp using the current memory cell's value as the coordinate (P = M).

### Jump to A — `'`

Warp to the coordinate specified by A (P = A).

### Jump to VS — `` ` ``

Warp back to the Visual Selection start coordinate (P = VS).

## Search

### Find Value — `n` / `N`

Warp forward / backward to a cell with the same value as A.

### Find Word — `w` / `W`

Warp forward / backward to the next non-zero cell.

### Find Zero — `z` / `Z`

Warp forward / backward to the next cell with a value of 0.

## Registers & Memory

### Zero A — `x`

Set A to 0.

### Zero M — `X`

Set the current memory cell M to 0.

### Zero Frame — `_`

Clear the entire current Z-frame (all 256 bytes) to 0.

### Swap M — `s`

Swap A with the current memory cell M.

### Swap S — `S`

Swap A with the secondary register S.

### Get P — `m`

Load the current coordinates into A (A = P).

### Set P — `M`

Write the current coordinates to the current memory cell (M = P).

### Replace / Fill — `r`

Fill the selected range (or current cell if not in Visual mode) with the value of A.

### Target Read — `t`

Read from the cell at the relative offset A from P into M (M = Mem[P + A]).

### Target Write — `T`

Write the value of M to the cell at the relative offset A from P (Mem[P + A] = M).

## Arithmetic & Logic

> Note: The primary result is stored in A. Byproducts like carry or remainder are automatically stored in S.

### Add — `+`

A = A + M.

### Sub — `-`

A = A - M.

### Mul — `*`

A = A \* M. The upper byte of the result is stored in S.

### Div — `/`

A = A / M. The remainder is stored in S. Division by zero is a no-op.

### AND — `&`

A = A & M (bitwise AND).

### OR — `|`

A = A | M (bitwise OR).

### XOR — `^`

A = A ^ M (bitwise XOR).

### NOT — `~`

A = ~A (bitwise inversion of all bits in A).

### Compare — `<` `>` `=`

Compare A with M. A = 1 if the condition is true, A = 0 if false.

### Dec — `,`

A = A - 1.

### Inc — `.`

A = A + 1.

### Logical NOT — `!`

A = 1 if A is 0, else A = 0.

### Binarize — `?`

A = 1 if A is non-zero, else A = 0.

### Shift Left / Right — `(` / `)`

Logical shift A left or right by 1 bit. The shifted-out bit is stored in S.

### Rotate Left / Right — `{` / `}`

Circular shift A left or right by 1 bit.

## Visual Mode & Clipboard

### Visual Mode — `v` / `V`

Start 1D / 2D rectangular visual selection mode. Memorizes the start coordinate as VS.

### Cancel — `g`

Cancel Visual mode and return to Normal mode.

### Yank — `y`

Copy the selected range to the yank buffer.

### Cut — `Y`

Copy the selected range to the yank buffer, then zero-clear the range.

### Paste — `p`

Paste the yank buffer starting at the current P.

### Paste at Origin — `P`

Paste the yank buffer starting at the origin (0, 0).

### Unpack — `u`

Unpack A's 8 bits into the selected range, placing each bit as 0 or 1 into each cell.

### Pack — `U`

Collect the LSB of each cell in the selected range and pack them into a single byte in A.

## Z-Stack & I/O

### Push — `[`

Move one Z-frame up while preserving the current P coordinates.

### Pop — `]`

Move one Z-frame down, returning to the preserved P coordinates.

### In Point — `i`

Read 1 byte from stdin into the current cell. Stores the number of bytes read in A (A = 0 on EOF).

### In Range — `I`

Read bytes from stdin into the selected range. Stores the number of bytes read in A (A = 0 on EOF).

### Out Point — `o`

Write 1 byte (the current cell) to stdout.

### Out Range — `O`

Write the selected range to stdout.

### Redirect — `$`

Switch the stdin source to the A-th command-line argument.

## Macro & Control Flow

### Dynamic Multiplier — `%`

Repeat the next instruction A times.

### Record — `q`

Start / stop recording a macro.

### Call — `@`

Call a recorded macro.

### Return — `R`

Exit the current macro immediately and return to the caller.

### String — `"`

Sequentially place the ASCII codes of the enclosed string into memory.

### Escape ASCII — `\`

Load the ASCII code of the immediately following character directly into A.

### Comment — `#`

Skip execution until the next newline.

### Query Depth — `Q`

Load the current macro nesting depth into A.

### Exit — `;`

Terminate the program using A as the exit status code.

### Command Mode — `:`

Reserved for runtime environment extended commands.

# 💡 Examples

16-bit Multiplication Macro
An example of a macro m that safely calculates a 16-bit multiplication using the Z-frame above as a "scratchpad" without side effects.

```
# --- Macro m definition ---
qm          # Start recording macro
[           # Move up one Z-axis layer
_           # Prologue: Clear the entire Z-frame
P           # Paste the arguments (2 bytes) from the yank buffer to origin (0,0)
H K s l * # Move to origin, swap 1st arg into A. Move right and multiply by 2nd arg
H s l S s   # Save lower byte (A) at origin, move right and save upper byte (S)
H v l y     # Epilogue: Yank the 2-byte result
]           # Move down one Z-axis layer (returns to original coordinates)
q           # Stop recording

# --- Caller ---
x 5 . s l   # M[0] = 5
x 7 . s     # M[1] = 7
H v l y     # Yank the 2 bytes
@m          # Call macro m
l l p       # Move right twice and paste the computed 2-byte result
;           # Exit
```

# ⚠️ Specifications & Edge Cases

- Stack Overflow: If the macro nesting depth or Z-axis depth exceeds 255 (i.e., attempting to transition to the 256th layer), the program will immediately terminate.
- EOF Handling: When reading via i or I, if the End-Of-File (EOF) is reached, the memory remains untouched and 0 is stored in the accumulator A.
- Division by Zero: If a division by zero occurs during / or %, the operation is voided, and the values of A and S are not updated.
