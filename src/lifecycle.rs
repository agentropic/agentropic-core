/// Agent lifecycle states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    /// Agent has been created but not initialized
    Created,
    /// Agent has been initialized
    Initialized,
    /// Agent is currently running
    Running,
    /// Agent execution is paused
    Paused,
    /// Agent has been stopped
    Stopped,
}