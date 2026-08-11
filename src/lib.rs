//! # core
//! Core primitives, traits, and abstractions for agent-oriented programming.


// Declare all modules
pub mod agent;
pub mod context;
pub mod error;
pub mod id;
pub mod identity;
pub mod lifecycle;
pub mod result;

// IMPORTANT: Declare prelude module
pub mod prelude;

// Re-export commonly used items at the crate root
pub use agent::Agent;
pub use context::{AgentContext, OutgoingMessage};
pub use error::AgentError;
pub use id::AgentId;
pub use identity::AgentIdentity;
pub use lifecycle::AgentState;
pub use result::AgentResult;
