use std::collections::HashMap;
use crate::types::*;
pub mod types;

pub struct ControlFlowGraph {
    /// The indice of the current block
    current_block: usize,
    /// The basic blocks found inside this given ControlFlowGraph
    blocks: Vec<BasicBlock>
}

impl ControlFlowGraph {
    /// Generates a ControlFlowGraph, starting at the given entry point address.
    fn new(entry_point: usize) -> Self {
        ControlFlowGraph { current_block: 0, blocks: vec![BasicBlock::new(entry_point)] }
    }

    /// Adds an edge to a BasicBlock, connecting src_block to dest_block.
    fn add_edge(&mut self, src_block: usize, dest_block: usize, traversed: bool) -> Result<(), CFGError> {
        let src_block = self.blocks.get_mut(src_block).ok_or(CFGError::MissingBlock)?;
        src_block.add_edge(dest_block, traversed);
        Ok(())
    }

    /// Adds a BasicBlock to the ControlFlowGraph and returns the position of the BasicBlock.
    fn add_block(&mut self, block: BasicBlock) -> usize {
        self.blocks.push(block);
        self.blocks.len() - 1
    }

    /// Searches for the block with the given start address and returns the position of it or creates a new one.
    fn query_block_or_create(&mut self, address: usize) -> usize {
        self.blocks.iter().position(|bb| bb.start == address).unwrap_or_else(|| { let new_block = BasicBlock::new(address); self.add_block(new_block) } )
    }

    /// Generates a dot file from the constructed ControlFlowGraph so far.
    pub fn dot(&self, filename: &str) { todo!() }

    /// Executes the given BlockType on the ControlFlowGraph
    pub fn execute(&mut self, program_counter: usize, instruction: BlockType) -> Result<(), CFGError> {
        match instruction {
            BlockType::Instruction(name, operand) => {
                let curr_block = self.blocks.get_mut(self.current_block).ok_or(CFGError::MissingCurrentBlock)?;
                assert!(program_counter >= curr_block.start, "Attempted to add an instruction behind the starting of the current block."); // TODO: Potentially generate an error-type?
                if !curr_block.block.contains_key(&program_counter) {
                    curr_block.add_instruction(program_counter, BlockType::Instruction(name, operand));
                }

                Ok(())
            }
            BlockType::Jump(name, success_address, jump_type, failure_address) => {
                // Add the instruction to the current block, if we already haven't
                let curr_block = self.blocks.get_mut(self.current_block).ok_or(CFGError::MissingCurrentBlock)?;
                assert!(program_counter >= curr_block.start, "Attempted to add an instruction behind the starting of the current block.");
                if !curr_block.block.contains_key(&program_counter) {
                    // NOTE: Should check if this creating a copy or just using move semantics to use the same thing of memory...
                    curr_block.add_instruction(program_counter, BlockType::Jump(name, success_address, jump_type, failure_address));
                }
                match jump_type {
                    JumpType::UnconditionalJump => {
                        let success_index = self.query_block_or_create(success_address);
                        self.add_edge(self.current_block, success_index, true)?;
                        self.current_block = success_index;
                        Ok(())
                    }
                    JumpType::ConditionalTaken => {
                        // Failure address needs to be defined.
                        let failure_address = failure_address.ok_or(CFGError::ExpectedFailureAddress)?;

                        let failure_index = self.query_block_or_create(failure_address);
                        self.add_edge(self.current_block, failure_index, false)?;
                        let success_index = self.query_block_or_create(success_address);
                        self.add_edge(self.current_block, success_index, true)?;
                        self.current_block = success_index;

                        Ok(())
                    }
                    JumpType::ConditionalNotTaken => {
                        // Failure address needs to be defined.
                        let failure_address = failure_address.ok_or(CFGError::ExpectedFailureAddress)?;

                        let failure_index = self.query_block_or_create(failure_address);
                        self.add_edge(self.current_block, failure_index, true)?;
                        let success_index = self.query_block_or_create(success_address);
                        self.add_edge(self.current_block, success_index, false)?;
                        self.current_block = failure_index;

                        Ok(())
                    }
                }
            }
        }
    }

}


struct BasicBlock {
    /// The starting address of this basic block.
    start: usize,
    /// The current end address of this basic block.
    end: usize,
    /// The mapping of each address to its respective BlockType.
    block: HashMap<usize, BlockType>,
    /// The edges for the given basic block which are indices to other BasicBlocks
    edges: Vec<(usize, usize)>
}

impl BasicBlock {
    /// Generates a new BasicBlock with a given start address
    fn new(start:usize) -> Self {
        BasicBlock { start: start, end: start, block: HashMap::new(), edges: Vec::new() }
    }

    /// Adds an instruction of BlockType to the given BasicBlock at the given address in the underlying HashMap.
    pub fn add_instruction(&mut self, address:usize, instruction: BlockType) {
        self.block.insert(address, instruction);
        self.end = address;
    }

    /// Returns an iterator of the current (key,value) pairs inside the underlying HashMap.
    pub fn iter(&self) -> impl Iterator<Item=(&usize, &BlockType)> {
        self.block.iter()
    }

    /// Adds a new edge if it cannot find it, otherwise increments the edge counter depending on if it was traversed or not.
    pub fn add_edge(&mut self, edge: usize, traversed: bool) {
        if let Some((_, cnt)) = self.edges.iter_mut().find(|(e, _)| *e == edge) {
            *cnt += traversed as usize;
        } else {
            self.edges.push((edge, traversed as usize));
        }
    }

}


// TODO: Write a bunch of test cases.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_flow_creation() {
        let cfg = ControlFlowGraph::new(2);
        // cfg.execute(3, instruction)
    }
}
