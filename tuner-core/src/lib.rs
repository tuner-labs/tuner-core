//! Tuner Core - Cross-platform radio station library
//! SPDX-License-Identifier: GPL-3.0-or-later

pub mod models;
pub mod error;
pub mod providers;
pub mod storage;

pub use error::{Error, Result};
pub use models::{Station, Tag};
pub use providers::{DataProvider, RadioBrowserProvider};
pub use storage::Storage;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
