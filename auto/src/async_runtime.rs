// ==================== Async runtime example ====================

// Simulates a network request that takes time to respond.
pub async fn fetch_network_request() -> u32 {
    // pauses THIS task for 1 second, letting other tasks run meanwhile
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    89
}
