//! Prelude module for convenient imports
//!
//! # Examples
//!
//! ```rust
//! use agentropic_core::prelude::*;
//! ```

// Re-export commonly used types and traits
pub use crate::agent::Agent;
pub use crate::id::AgentId;
pub use crate::identity::AgentIdentity;
pub use crate::context::{AgentContext, OutgoingMessage};
pub use crate::error::AgentError;
pub use crate::result::AgentResult;
pub use crate::lifecycle::AgentState;

// Re-export async-trait for convenience
pub use async_trait::async_trait;
