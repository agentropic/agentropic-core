use async_trait::async_trait;
use crate::{AgentId, AgentContext, AgentResult};

/// Core agent trait that all agents must implement
#[async_trait]
pub trait Agent: Send + Sync {
    /// Returns the unique identifier for this agent
    fn id(&self) -> &AgentId;
    
    /// Initialize the agent
    async fn initialize(&mut self, ctx: &AgentContext) -> AgentResult<()>;
    
    /// Execute the agent's main behavior
    async fn execute(&mut self, ctx: &AgentContext) -> AgentResult<()>;
    
    /// Gracefully shutdown the agent
    async fn shutdown(&mut self, ctx: &AgentContext) -> AgentResult<()>;
}