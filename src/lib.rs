//! # agentropic-core
//!
//! Core primitives, traits, and abstractions for agent-oriented programming.

pub mod agent;
pub mod id;
pub mod identity;
// pub mod metadata;
pub mod context;
// pub mod state;
// pub mod factory;
pub mod error;
pub mod result;
pub mod lifecycle;

// Add prelude module
pub mod prelude;

// Re-exports at the root level
pub use agent::Agent;
pub use id::AgentId;
pub use identity::AgentIdentity;
pub use context::AgentContext;
pub use error::AgentError;
pub use result::AgentResult;
pub use lifecycle::AgentState;