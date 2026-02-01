//! UI Module
//!
//! Terminal user interface using ratatui.

pub mod components;
pub mod renderer;

// Re-exports
pub use components::{
    MessageType,
    PasswordDialog,
};
