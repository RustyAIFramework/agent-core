# agent-core

[![Crates.io](https://img.shields.io/crates/v/agent-core.svg)](https://crates.io/crates/agent-core)
[![Documentation](https://docs.rs/agent-core/badge.svg)](https://docs.rs/agent-core)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**Core primitives, traits, and abstractions for agent-oriented programming in Rust.**

`agent-core` is the foundational crate of the RustyAI ecosystem, providing the building blocks for creating autonomous, intelligent agents. It defines agent identity, lifecycle management, core traits, and fundamental abstractions that all other RustyAI crates build upon.

---

## Purpose

This crate establishes:

- **Agent Identity**: Unique identification and addressing for agents
- **Agent Traits**: Core behavioral interfaces that all agents implement
- **Lifecycle Management**: Agent creation, initialization, execution, and termination
- **Core Abstractions**: Fundamental types and patterns for agent-oriented systems

---

## Core Concepts

### Agent Identity

Every agent has a unique identity that enables:
- Addressability in multi-agent systems
- Message routing and communication
- Access control and permissions
- Tracking and observability
```rust
use agent_core::{AgentId, AgentIdentity};

let agent_id = AgentId::new();
let identity = AgentIdentity::new(agent_id, "trading-agent");
```

### Agent Trait

The `Agent` trait is the fundamental interface all agents must implement:
```rust
use agent_core::{Agent, AgentContext, AgentResult};

#[async_trait]
pub trait Agent: Send + Sync {
    /// Unique identifier for this agent
    fn id(&self) -> &AgentId;
    
    /// Initialize the agent
    async fn initialize(&mut self, ctx: &AgentContext) -> AgentResult<()>;
    
    /// Execute the agent's main behavior
    async fn execute(&mut self, ctx: &AgentContext) -> AgentResult<()>;
    
    /// Gracefully shutdown the agent
    async fn shutdown(&mut self, ctx: &AgentContext) -> AgentResult<()>;
}
```

### Agent Lifecycle

Agents follow a well-defined lifecycle:

1. **Creation** → Agent instance is constructed
2. **Initialize** → Resources allocated, connections established
3. **Execute** → Main agent behavior runs
4. **Shutdown** → Cleanup and graceful termination

The lifecycle is managed by the runtime (see `runtime`).

### Agent Context

`AgentContext` provides agents with access to:
- Configuration
- Logging and telemetry
- Shared resources
- Communication channels

---

## What's Included

### Core Types

- `AgentId` - Unique agent identifier (UUID-based)
- `AgentIdentity` - Full identity with ID and human-readable name
- `AgentMetadata` - Descriptive metadata about an agent
- `AgentState` - Lifecycle state tracking

### Traits

- `Agent` - Core agent behavior
- `AgentFactory` - Agent construction and dependency injection
- `Perceivable` - Agents that can perceive their environment
- `Actionable` - Agents that can take actions

### Result Types

- `AgentResult<T>` - Standard result type for agent operations
- `AgentError` - Comprehensive error types for agent failures

### Utilities

- Agent lifecycle state machines
- Identity generation and validation
- Context management helpers

---

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
agent-core = "0.1.0"
```

### Basic Agent Implementation
```rust
use agent_core::{Agent, AgentId, AgentContext, AgentResult};
use async_trait::async_trait;

pub struct SimpleAgent {
    id: AgentId,
    counter: u64,
}

impl SimpleAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId::new(),
            counter: 0,
        }
    }
}

#[async_trait]
impl Agent for SimpleAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    async fn initialize(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info("SimpleAgent initializing");
        Ok(())
    }

    async fn execute(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        self.counter += 1;
        ctx.log_info(&format!("Execution count: {}", self.counter));
        Ok(())
    }

    async fn shutdown(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info("SimpleAgent shutting down");
        Ok(())
    }
}
```

---

## Architecture

`agent-core` is designed to be:

- **Minimal**: Only fundamental abstractions, no opinions on implementation
- **Extensible**: Traits allow for diverse agent types
- **Safe**: Leverages Rust's type system for correctness
- **Async-first**: Built for concurrent, non-blocking agent execution

---

## 🔗 Related Crates

- **[messaging](../messaging)** - Agent communication protocols
- **[cognition](../cognition)** - Reasoning and decision-making
- **[runtime](../runtime)** - Agent execution engine
- **[rustyai](../rustyai)** - Batteries-included facade

---

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/agent-core).

For guides and tutorials, see [docs](https://github.com/rustyai/docs).

---

## Contributing

Contributions are welcome! Please see the [contributing guidelines](../../CONTRIBUTING.md).

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## Status

**Active Development** - This crate is under active development. APIs may change before 1.0 release.

---

*Part of the [RustyAI](https://github.com/rustyai) ecosystem for agent-oriented programming in Rust.*