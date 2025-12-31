use crate::AgentId;

/// Agent identity with ID and name
#[derive(Debug, Clone)]
pub struct AgentIdentity {
    id: AgentId,
    name: String,
}

impl AgentIdentity {
    /// Create a new agent identity
    pub fn new(id: AgentId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
    
    /// Get the agent ID
    pub fn id(&self) -> &AgentId {
        &self.id
    }
    
    /// Get the agent name
    pub fn name(&self) -> &str {
        &self.name
    }
}