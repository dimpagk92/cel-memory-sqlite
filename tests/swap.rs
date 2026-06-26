//! Crate-level conformance test: `SqliteMemoryProvider` must work through
//! the locked `cel_memory::MemoryProvider` trait surface, not just as a
//! concrete type.
//!
//! Runtime-level integration tests that need policy engines or action gateways
//! should live downstream. This crate deliberately keeps dev-dependencies small
//! so it remains standalone-testable.

use std::sync::Arc;

use cel_memory::{
    assert_retrieve_finds_written, assert_session_lifecycle, assert_summarize_session_roundtrip,
    assert_write_get_stats, MemoryProvider, MockSummarizer,
};
use cel_memory_sqlite::{MockEmbedder, SqliteMemoryProvider};

#[tokio::test]
async fn sqlite_provider_works_through_locked_trait() {
    let embedder = Arc::new(MockEmbedder::new());
    let summarizer = MockSummarizer::new("session synthesis");
    let memory: Arc<dyn MemoryProvider> = Arc::new(
        SqliteMemoryProvider::open_in_memory(embedder)
            .await
            .unwrap()
            .with_summarizer(summarizer),
    );

    let (_chunk, stats) = assert_write_get_stats(memory.clone(), "user asked about the Q4 report")
        .await
        .unwrap();
    assert_eq!(stats.total_chunks, 1);
    assert_eq!(stats.embedding_model.as_deref(), Some("mock-384"));

    assert_retrieve_finds_written(memory.clone(), "Q4 revenue forecast")
        .await
        .unwrap();
    assert_session_lifecycle(memory.clone()).await.unwrap();
    assert_summarize_session_roundtrip(memory).await.unwrap();
}
