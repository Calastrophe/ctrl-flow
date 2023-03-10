use thiserror::Error;


#[derive(Clone, Copy)]
pub enum JumpType {
    UnconditionalJump,
    ConditionalTaken,
    ConditionalNotTaken
}

pub enum BlockType {
    Instruction(String, String),
    Jump(String, usize, JumpType, Option<usize>)
}

#[derive(Error, Debug)]
pub enum CFGError {
    #[error("There was an attempt to find a BasicBlock which doesn't exist.")]
    MissingBlock,
    #[error("The current block does not exist inside the BasicBlocks.")]
    MissingCurrentBlock,
    #[error("A failure address was expected for a conditional jump and it was not provided.")]
    ExpectedFailureAddress,
}