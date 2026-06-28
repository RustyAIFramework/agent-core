// Define a concrete agent for testing

use z_core::prelude::*;

struct TestAgent {
    id: AgentId,
}

impl TestAgent {
    fn new() -> Self {
        Self {
            id: AgentId::new(),
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
    assert!(!agent.id().to_string().is_empty());
}

#[test]
fn agent_has_unique_id() {
    let agent1 = TestAgent::new();
    let agent2 = TestAgent::new();
    assert_ne!(agent1.id(), agent2.id());
}

#[tokio::test]
async fn test_agent_lifecycle() {
    let mut agent = TestAgent::new();
    let ctx = AgentContext::new(*agent.id());
    assert!(agent.initialize(&ctx).await.is_ok());
    assert!(agent.execute(&ctx).await.is_ok());
    assert!(agent.shutdown(&ctx).await.is_ok());
}
