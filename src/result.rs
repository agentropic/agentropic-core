use crate::AgentError;

/// Result type for agent operations
pub type AgentResult<T> = Result<T, AgentError>;