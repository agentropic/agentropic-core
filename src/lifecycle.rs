/// Agent lifecycle states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Created,
    Initialized,
    Running,
    Paused,
    Stopped,
}