use thiserror::Error;

/// Agent errors
#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Shutdown failed: {0}")]
    ShutdownFailed(String),
}
