pub mod radio_browser;
pub use radio_browser::RadioBrowserProvider;

use crate::models::Station;
use crate::Result;

pub trait DataProvider: Send + Sync {
    fn search(&self, text: &str) -> Result<Vec<Station>>;
    fn name(&self) -> String;
}
