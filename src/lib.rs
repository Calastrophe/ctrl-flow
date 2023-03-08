use thiserror::Error;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum JumpType {
    UnconditionalJump,
    ConditionalTaken,
    ConditionalNotTaken
}

#[derive(Clone)]
enum BlockType {
    Instruction(String, String),
    Jump(String, usize, JumpType, Option<usize>)
}

#[derive(Error, Debug)]
enum CFGError {
    #[error("There was an attempt to find a BasicBlock which doesn't exist.")]
    MissingBlock,
    #[error("The current block does not exist inside the BasicBlocks.")]
    MissingCurrentBlock,
    #[error("A failure address was expected for a conditional jump and it was not provided.")]
    ExpectedFailureAddress,
}


struct ControlFlowGraph {
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
    fn add_edge(&mut self, src_block: usize, dest_block: usize) -> Result<(), CFGError> {
        let Some(_dest) = self.blocks.get(dest_block) else {
            return Err(CFGError::MissingBlock)
        };

        if let Some(src_block) = self.blocks.get_mut(src_block) {
            src_block.add_edge(dest_block);
        } else {
            return Err(CFGError::MissingBlock)
        }

        Ok(())
    }

    /// Adds a BasicBlock to the ControlFlowGraph and returns the position of the BasicBlock, if needed.
    fn add_block(&mut self, block: BasicBlock) -> usize {
        self.blocks.push(block);
        self.blocks.len() - 1
    }

    /// Linearly searches the blocks currently in the graph to identify if the given block conflicts with another.
    fn query_blocks(&self, address: usize) -> Option<usize> {
        for (i, bb) in self.blocks.iter().enumerate() {
            if bb.start == address {
                return Some(i)
            }
        }
        None
    }

    pub fn execute(&mut self, program_counter: usize, instruction: BlockType) -> Result<(), CFGError> {

        match instruction {
            BlockType::Instruction(name, operand) => {
                let Some(curr_block) = self.blocks.get_mut(self.current_block) else {
                    return Err(CFGError::MissingCurrentBlock)
                };

                if program_counter >= curr_block.start && program_counter <= curr_block.end {
                    curr_block.add_instruction(program_counter, BlockType::Instruction(name, operand));
                }

                Ok(())
            }
            // TODO: Refactor this jump_type match to not be so horribly written...
            BlockType::Jump(name, success_address, jump_type, failure_address) => {
                // Add the instruction to the current block, if we already haven't
                let curr_block = self.blocks.get_mut(self.current_block).ok_or(CFGError::MissingCurrentBlock)?;
                if program_counter >= curr_block.start && program_counter <= curr_block.end {
                    curr_block.add_instruction(program_counter, BlockType::Jump(name, success_address, jump_type, failure_address));
                }

                match jump_type {
                    JumpType::UnconditionalJump => {

                        if let Some(success_index) = self.query_blocks(success_address) {
                            self.add_edge(self.current_block, success_index)?;
                            self.current_block = success_index;
                        } else {
                            let new_block = BasicBlock::new(success_address);
                            let success_index = self.add_block(new_block);
                            self.current_block = success_index;
                        }
                        Ok(())
                    }
                    JumpType::ConditionalTaken => {
                        // Failure address needs to be defined.
                        let Some(fail_address) = failure_address else {
                            return Err(CFGError::ExpectedFailureAddress)
                        };

                        // Does a failure block already exist?
                        if let Some(failure_index) = self.query_blocks(fail_address) {
                            self.add_edge(self.current_block, failure_index)?;
                        } else { // The failure block does not exist
                            let failure_block = BasicBlock::new(fail_address);
                            let failure_index = self.add_block(failure_block);
                            self.add_edge(self.current_block, failure_index)?;
                        }

                        // Does a success block already exist?
                        if let Some(success_index) = self.query_blocks(success_address) {
                            self.add_edge(self.current_block, success_index)?;
                            // The conditional was taken, so only change the current block in the success case.
                            self.current_block = success_index;
                        } else { // The success block does not exist
                            let new_block = BasicBlock::new(success_address);
                            let success_index = self.add_block(new_block);
                            self.add_edge(self.current_block, success_index)?;
                            // The conditional was taken, so only change the current block in the success case.
                            self.current_block = success_index;
                        };

                        Ok(())
                    }
                    JumpType::ConditionalNotTaken => {
                        let Some(fail_address) = failure_address else {
                            return Err(CFGError::ExpectedFailureAddress)
                        };

                        // Does the success block already exist?
                        if let Some(success_index) = self.query_blocks(success_address) {
                            self.add_edge(self.current_block, success_index)?;
                        } else { // The success block does not exist
                            let new_block = BasicBlock::new(success_address);
                            let success_index = self.add_block(new_block);
                            self.add_edge(self.current_block, success_index)?;
                        };

                        // Does the failure block already exist?
                        if let Some(failure_index) = self.query_blocks(fail_address) {
                            self.add_edge(self.current_block, failure_index)?;
                            // The conditional was not taken, only change in the false case.
                            self.current_block = failure_index;
                        } else { // The failure block does not exist
                            let failure_block = BasicBlock::new(fail_address);
                            let failure_index = self.add_block(failure_block);
                            self.add_edge(self.current_block, failure_index)?;
                            // The conditional was not taken, only change in the false case.
                            self.current_block = failure_index;
                        }

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
    edges: Vec<usize>
}

impl BasicBlock {
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

    pub fn add_edge(&mut self, edge: usize) {
        self.edges.push(edge);
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
