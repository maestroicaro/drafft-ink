//! Widget system for managing interactive shape states.
//!
//! This module provides a unified way to manage UI state for shapes:
//! - Selection state with shape-specific handles
//! - Editing state (e.g., text editing)
//! - Event routing to the appropriate handler
//!
//! Shapes remain pure data. Widgets wrap shapes with UI state.

mod state;
mod manager;
mod handles;

pub use state::{WidgetState, EditingKind};
pub use manager::WidgetManager;
pub use handles::{Handle, HandleKind, HandleShape};
