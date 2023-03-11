use thiserror::Error;


#[derive(Clone, Copy)]
pub enum JumpType {
    UnconditionalJump,
    ConditionalTaken,
    ConditionalNotTaken
}

pub enum BlockType {
    Instruction(String, Option<String>),
    Jump(String, usize, JumpType, Option<usize>)
}

impl ToString for BlockType {
    fn to_string(&self) -> String {
        match self {
            BlockType::Instruction(name, operand) => {
                format!("{} {}", name, operand.clone().unwrap_or("".to_string()))
            }
            BlockType::Jump(name, success_address, jump_type, failure_address) => {
                format!("{} {}", name, success_address)
            }
        }
    }
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