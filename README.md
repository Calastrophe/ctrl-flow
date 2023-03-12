## What is ctrl-flow?

ctrl-flow is a library which makes easy to implement control flow graphs.

It is more targetted toward creators of virtual machines, interpreters, or emulators.

This is just a bare-bones framework to construct a control flow graph, then it is up to the user to export it to another format. There will be some provided formats, but hidden behind feature flags in future releases.


**Pull requests are welcome for optimizing or improving the library**.

## How do I use it?

Firstly, there are only two types which can act on the control flow graph. `Instruction` and `Jump`.

You must convert your instruction which you are executing at run-time into a `BlockType` depending on how the instruction effects control flow. `Instruction` contains a name and optionally an operand. A `Jump` contains a name, success address, `JumpType`, and failure address. The failure address is only required in the case of a conditional `JumpType`.

Your `JumpType` in a `Jump` can be one of three: `UnconditionalJump`, `ConditionalTaken`, and `ConditionalNotTaken`.

After you've constructed the `BlockType`, you can then use the execute command with the current program counter to take an effect on the graph.

To export the ControlFlowGraph, you can use the provided iterators to export into your own format. In future releases, these will be provided, but opt-in.

### Non-goals

Implementing and maintaining five thousand different formats to output

Switch statements? *maybe...*



