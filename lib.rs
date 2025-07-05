pub mod blockchain;
pub mod network;
pub mod storage;

pub use blockchain::*;
pub use network::*;
pub use storage::*;

/// Initializes the blockchain library and connects with backend components.
pub fn initialize() {
    // Example: Connect to storage backend
    storage::connect();

    // Example: Initialize blockchain state
    blockchain::init();

    // Example: Start network services
    network::start();
}