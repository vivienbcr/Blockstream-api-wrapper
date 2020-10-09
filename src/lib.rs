pub use reqwest;
pub use serde;
// use std::collections::HashMap;
// use serde::Deserialize;
pub mod data;
pub mod async_impl;
#[cfg(feature = "blocking")]
pub mod blocking;
