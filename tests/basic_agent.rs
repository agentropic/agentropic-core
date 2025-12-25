use agentropic_core::prelude::*;

#[test]
fn create_agent() {
    let agent = Agent::new();
    assert!(agent.beliefs.is_empty());
    assert!(agent.goals.is_empty());
}
