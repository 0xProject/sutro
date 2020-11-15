use thiserror::Error;

#[derive(Clone, Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("pop on empty stack")]
    StackUnderflow,

    #[error("stack to large")]
    StackOverflow,

    #[error("invalid opcode executed")]
    InvalidOpcode,

    #[error("jump to invalid destination")]
    InvalidJump,

    #[error("jump to unknown destination")]
    ControlFlowEscaped,
}
