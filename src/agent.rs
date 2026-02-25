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

    /// Handle an incoming message from another agent
    /// Default implementation does nothing — override to handle messages
    async fn handle_message(
        &mut self,
        ctx: &AgentContext,
        sender: &str,
        performative: &str,
        content: &str,
    ) -> AgentResult<()> {
        // Default: ignore messages
        let _ = (ctx, sender, performative, content);
        Ok(())
    }
}
