/// Agent git status
#[derive(Debug)]
pub struct AgentContext {
    // TODO: Add fields for logging, metrics, etc.
}

impl AgentContext {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn log_info(&self, msg: &str) {
        println!("[INFO] {}", msg);
    }
}

impl Default for AgentContext {
    fn default() -> Self {
        Self::new()
    }
}