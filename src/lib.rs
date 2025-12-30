//! # agentropic-core
//!
//! Core primitives, traits, and abstractions for agent-oriented programming.

pub mod agent;
pub mod id;
pub mod identity;
pub mod context;
pub mod lifecycle;
pub mod error;
pub mod result;

// Re-exports
pub use agent::Agent;
pub use id::AgentId;
pub use identity::AgentIdentity;
pub use context::AgentContext;
pub use error::AgentError;
pub use result::AgentResult;