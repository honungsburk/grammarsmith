//! Position utilities.
//!
//! This module provides utilities for working with positions in a file.
//!
//! # Examples
//! ```
//! use grammarsmith::position::*;
//! ```

pub mod bytepos;
pub mod lineoffset;
pub mod span;

pub use bytepos::*;
pub use lineoffset::*;
pub use span::*;
