# ctrl-flow

### This library is a work-in-progress, things may not be finished and are **subject to change**.

This is a library for easily implementing control flow graphs in Rust.

This is more targetted towards custom architectures with emulators, virtual machines, or some type of interpreter.

The aim of this library is to avoid needless dependecies and just deliver the user of this library what is needed.

**Pull requests are welcome for optimizing or improving the library**.

### Implementation information

Currently, there are two types which your instruction set needs to be formatted into - Instruction or Jump.

An instruction is just the name of the instruction followed by the potential operand.

A jump contains similarly the name and the operand (success address), followed by the jump type, then the failure address.

A jump type is one of three: UnconditionalJump, ConditionalTaken, ConditionalNotTaken

These things are needed for cfg-rs to accurately transcribe your control flow without needing to know the details of your architecture.
