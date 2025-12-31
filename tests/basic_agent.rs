use agentropic_core::prelude::*;

// Helper struct for testing
struct TestAgent {
    id: AgentId,
    beliefs: Vec<String>,
    goals: Vec<String>,
}

impl TestAgent {
    fn new() -> Self {
        Self {
            id: AgentId::new(),
            beliefs: Vec::new(),
            goals: Vec::new(),
        }
    }
}

#[async_trait]
impl Agent for TestAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    async fn initialize(&mut self, _ctx: &AgentContext) -> AgentResult<()> {
        Ok(())
    }

    async fn execute(&mut self, _ctx: &AgentContext) -> AgentResult<()> {
        Ok(())
    }

    async fn shutdown(&mut self, _ctx: &AgentContext) -> AgentResult<()> {
        Ok(())
    }
}

#[test]
fn create_agent() {
    let agent = TestAgent::new();
    assert!(agent.beliefs.is_empty());
    assert!(agent.goals.is_empty());
}

#[test]
fn agent_has_id() {
    let agent = TestAgent::new();
    let id = agent.id();
    assert_eq!(id, agent.id());
}

#[test]
fn add_beliefs() {
    let mut agent = TestAgent::new();
    agent.beliefs.push("belief1".to_string());
    assert_eq!(agent.beliefs.len(), 1);
}

#[test]
fn add_goals() {
    let mut agent = TestAgent::new();
    agent.goals.push("goal1".to_string());
    assert_eq!(agent.goals.len(), 1);
}