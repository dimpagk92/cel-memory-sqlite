//! Re-exports the [`Embedder`] trait from [`cel_memory`].
//!
//! The trait and [`MockEmbedder`] live in `cel-memory` so custom backends can
//! implement embedders without depending on SQLite. This module preserves the
//! historical import path (`cel_memory_sqlite::Embedder`).

pub use cel_memory::{Embedder, EmbedderError, EmbedderResult, MockEmbedder};

/// Historical alias used by the `fastembed` backend module.
pub type EmbedResult<T> = EmbedderResult<T>;
