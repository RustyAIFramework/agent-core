//! Agent context for runtime information and messaging

use std::sync::{Arc, Mutex};
use crate::AgentId;

/// An outgoing message: (receiver_id, performative, content)
#[derive(Debug, Clone)]
pub struct OutgoingMessage {
    pub receiver: String,
    pub performative: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct AgentContext {
    agent_id: AgentId,
    outbox: Arc<Mutex<Vec<OutgoingMessage>>>,
}

impl AgentContext {
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            agent_id,
            outbox: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }

    /// Send a message to another agent (queued, delivered by runtime)
    pub fn send_message(&self, receiver: &str, performative: &str, content: &str) {
        if let Ok(mut outbox) = self.outbox.lock() {
            outbox.push(OutgoingMessage {
                receiver: receiver.to_string(),
                performative: performative.to_string(),
                content: content.to_string(),
            });
        }
    }

    /// Drain outgoing messages (called by runtime)
    pub fn drain_outbox(&self) -> Vec<OutgoingMessage> {
        if let Ok(mut outbox) = self.outbox.lock() {
            outbox.drain(..).collect()
        } else {
            Vec::new()
        }
    }

    pub fn log_info(&self, msg: &str) {
        println!("[INFO] [{}] {}", self.agent_id, msg);
    }

    pub fn log_error(&self, msg: &str) {
        eprintln!("[ERROR] [{}] {}", self.agent_id, msg);
    }
}
