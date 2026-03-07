# bed (Binary EDitor)
bed is an esoteric programming language (Esolang) and its execution environment, heavily inspired by Vim's keybindings and operational philosophy.
It features an unorthodox and mind-bending Instruction Set Architecture (ISA) that allows you to navigate a 2D binary space like a text editor, select ranges (yank), perform calculations, and traverse Z-axis stack frames.

# 🌌 Architecture

The bed virtual machine has the following architecture:

* Main Memory: A 16 × 16 byte (256 bytes) 2D torus space. The top, bottom, left, and right edges wrap around.
* Z-Stack: The main memory is stacked along the Z-axis, consisting of 256 layers (frames) from 0 to 255.
* Pointer (P): The current X coordinate (4 bits) and Y coordinate (4 bits).
* Accumulator (A): The primary 1-byte (8-bit) register for arithmetic and logic operations.
* Secondary (S): A 1-byte register used to store the byproducts of operations (e.g., carry, remainder, upper byte of multiplication) or for temporary data preservation.
* Yank Buffer: A clipboard that holds data from ranges selected in Visual mode.

# 🚀 Modes

The runtime operates in two modes:

* Interactive Mode (TUI): Launched via the bed command without arguments. It acts as a full-screen binary editor, allowing you to code and execute while visualizing the memory space and registers in real-time.
* Batch Mode: Launched via bed script.bed. It runs headlessly without rendering the UI, acting as a pure, high-speed interpreter utilizing only standard I/O.

# ⌨️ Instruction Set Architecture (ISA)

1. Prefixes

| Key | Action |
|---|---|
| 0-9, a-f, A-F | Hex Multiplier: Repeats the immediately following instruction the specified number of times in hexadecimal. |

2. Motion & Jump

| Key | Action |
|---|---|
| h j k l | Relative Move: Move left, down, up, right (wraps around the torus). |
| H J K L | Edge Move: Absolute jump to the edges (X=0, Y=15, Y=0, X=15 respectively). |
| G | Deref: Warp using the current memory cell's value as the coordinate (P = M). |
| ' | Jump to A: Warp to the coordinate specified by A (P = A). |
| ` | Jump to VS: Warp back to the Visual Selection start coordinate. |

3. Search

| Key | Action |
|---|---|
| n / N | Find Value: Warp forward/backward to a cell with the same value as A. |
| w / W | Find Word: Warp forward/backward to the next non-zero cell. |
| z / Z | Find Zero: Warp forward/backward to the next cell with a value of 0. |

4. Registers & Memory

| Key | Action |
|---|---|
| x / X | Zero A / M: Set A to 0 / Set the current memory cell to 0. |
| _ | Zero Frame: Clear the entire current Z-frame (all 256 bytes) to 0. |
| s / S | Swap: Swap A with current memory M / Swap A with secondary S. |
| m / M | Get / Set P: A = P (load coordinates into A) / M = P (write coordinates to M). |
| r | Replace / Fill: Fill the selected range (or current cell) with the value of A. |
| t / T | Target Read / Write: Read from / Write to the cell at the relative offset A from P (Mem[P + A]). |

5. Arithmetic & Logic

Note: The primary result is stored in A. Byproducts like carry or remainder are automatically stored in S.

| Key | Action |
|---|---|
| + - * / | Arithmetic: Add, Subtract, Multiply, Divide (division by zero is a no-op). |
| & | ^ ~ | Bitwise: AND, OR, XOR, NOT (bitwise inversion). |
| < > = | Compare: Compare A with M. A=1 if true, A=0 if false. |
| , / . | Dec / Inc: A = A - 1 / A = A + 1. |
| ! / ? | Logical NOT / Binarize: A=1 if 0, else A=0 / A=1 if non-zero. |
| ( / ) | Shift L/R: Logical shift on A (the shifted-out bit goes to S). |
| { / } | Rotate L/R: Circular shift on A. |

6. Visual Mode & Clipboard (Yank)

| Key | Action |
|---|---|
| v / V | Visual Mode: Start 1D / 2D rectangular visual mode. Memorizes the start coordinate (VS). |
| g | Cancel: Cancel Visual mode and return to Normal mode. |
| y / Y | Yank / Cut: Copy the selected range to the buffer / Copy and zero-clear the range. |
| p / P | Paste: Paste starting at the current P / Paste starting at the origin (0,0). |
| u / U | Unpack / Pack: Unpack A's 8 bits into the selected range as 0s and 1s / Pack the LSB of each cell in the range into A. |

7. Z-Stack & I/O

| Key | Action |
|---|---|
| [ / ] | Push / Pop: Move one Z-frame up / down while preserving the P coordinates. |
| i / I | In Point/Range: Read 1 byte / range from stdin. Number of bytes read goes to A (A=0 on EOF). |
| o / O | Out Point/Range: Write 1 byte / range to stdout. |
| $ | Redirect: Switch stdin source to the A-th command-line argument. |

8. Macro & Control Flow

| Key | Action |
|---|---|
| % | Dynamic Multiplier: Repeat the next instruction A times. |
| q / @ | Record / Call: Start/stop recording a macro / Call a macro. |
| R | Return: Exit the current macro immediately and return to the caller. |
| " | String: Sequentially place the ASCII codes of the string enclosed in " into memory. |
| \ | Escape ASCII: Load the ASCII code of the immediately following character directly into A. |
| # | Comment: Skip execution until the next newline. |
| Q | Query Depth: Load the current macro nesting depth into A. |
| ; | Exit: Terminate the program using A as the exit status code. |
| : | Command Mode: Reserved for runtime environment extended commands. |

# 💡 Examples

16-bit Multiplication Macro
An example of a macro m that safely calculates a 16-bit multiplication using the Z-frame above as a "scratchpad" without side effects.

```
# --- Macro m definition ---
qm          # Start recording macro
[           # Move up one Z-axis layer
_           # Prologue: Clear the entire Z-frame
P           # Paste the arguments (2 bytes) from the yank buffer to origin (0,0)
H s l * # Move to origin, swap 1st arg into A. Move right and multiply by 2nd arg
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
* Stack Overflow: If the macro nesting depth or Z-axis depth exceeds 255 (i.e., attempting to transition to the 256th layer), the program will immediately terminate.
* EOF Handling: When reading via i or I, if the End-Of-File (EOF) is reached, the memory remains untouched and 0 is stored in the accumulator A.
* Division by Zero: If a division by zero occurs during / or %, the operation is voided, and the values of A and S are not updated.
