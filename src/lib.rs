pub use reqwest;
pub use serde;
// use std::collections::HashMap;
// use serde::Deserialize;
pub mod data;
pub(crate) use self::data::{blockstream};
pub mod async_impl;
pub use self::async_impl::{client,reqwests};
// pub use crate::data::blockstream;

// #[cfg(feature = "blocking")]
// pub mod data;
// pub mod blocking;
