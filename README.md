# ZenOS
x64 uefi rust operating system centered around it's own language

> [!WARNING]
> Designs below are not fully implemented, they are concepts and can change in the future !

## Design of ZenOS
### Layers
- Bottom: Rust kernel
- Middle: ZenLang Virtual Machine
- Top: ZenLang

The lower the level, the more control over system you have.

This design allows for tunable executable priveleges. For example, you can make certain executables run at kernel, or at user level

Why are ZenLang VM and ZenLang seperate levels? The kernel controls the VM, kernel runs the executables, performs multithreading, ZenLang on the other hand does not do all of that. Joining these levels together would mean the kernel controls ZenLang itself, which is not what it does

## Design of ZenLang
ZenLang code is compiled to series of opcodes, which we call "modules".
Modules can be dynamically loaded at any time of the program execution

A module contains:
- Functions names, adresses, argument counts
- Opcodes

## Design of ZenLang Virtual Machine
ZenLang VM is a general step based VM, it has a program counter, it executes opcodes, etc...
However, ZenLang VM doesn't have registers. Everything is done by pushing/poping from the stack. Why? - ZenLang is dynamically typed by design, having registers would cause a lot of problems. Additionally, it's just easier to manage values this way

For example, to add 2 numbers together, we would do:
```
Loadcn(5),
Loadcn(5),
Add(),
```
Add() instruction pops two numbers loaded by `Loadcn()` and adds them together, then pushes the result to the stack

Here's another example:
```
Loadcn(2),
Loadv("var"),
Div(),
```
This divides the loaded variable `var` by two. Does not affect the actual value stored in `var`

### Stacks
ZenLang VM has 2 stacks:
- Main stack
  - Contains values that can either be a Number, String, Boolean, FunctionRef, etc...
  - Affected by: `Loadcn`, `Loadcnu`, `Loadcs`, `Loadv`, `Storev`, etc...
- Call stack (or return stack)
  - Contains saved program counter by call. Pop'ed on Ret instruction
  - Affected by: `Call`, `Ret`

### Program counter
As you may know, ZenLang has a concept called modules, which can dynamically loaded at runtime

However, re-linking the entire executable at runtime is expensive, this is why the program counter is divided into two halfs:

- Low 32 bit half: Stores the opcode address relative to the module
- High 32 bit half: Stores the module ID

Later opcodes are fetched like this (pseudocode, not following rust's borrow rules):
```rust
let opcode = self.modules[self.pc.get_high()].opcodes[self.pc.get_low()]
```

### Scopes
Scopes are embedded into ZenLang VM, which is why we have instructions like Loadv and Storev

Scopes are created on `Call()`

Last scope is destroyed on `Ret()`

`Loadv` and `Storev` only load/store variables from the last scope. They can't access other scopes
