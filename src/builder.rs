//! # Implementing New Agents
//!
//! To implement a new agent, you need to:
//! 1. Create a new agent type
//! 2. Implement required traits
//! 3. Create a public constructor function
//!
//! ## Example Implementation
//!
//! ```no_compile
//! // 1. Define your agent type
//! pub struct MyNewAgent;
//!
//! // 2. Implement required traits
//! impl SimpleAgent for MyNewAgent {
//!     fn builder(self) -> AgentBuilder<Simple> {
//!         AgentBuilder::new()
//!     }
//! }
//!
//! // Optionally implement TaggedAgent if your source supports tags
//! impl TaggedAgent for MyNewAgent {
//!     fn tagged_builder(self) -> AgentBuilder<Tagged> {
//!         AgentBuilder::new()
//!     }
//! }
//!
//! // 3. Create a public constructor
//! pub fn my_new_agent() -> MyNewAgent {
//!     MyNewAgent
//! }
//! ```

use crate::{Agent, Result};
use std::marker::PhantomData;

// Маркерные типы для билдера
pub struct Simple;
pub struct Tagged;
