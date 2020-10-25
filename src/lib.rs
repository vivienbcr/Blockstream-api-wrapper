pub use reqwest;
pub use serde;
pub mod data;
pub mod async_impl;
pub use self::async_impl::{client,reqwests};

// #[cfg(feature = "blocking")]
pub mod blocking;
pub use self::blocking::ApiClient;
