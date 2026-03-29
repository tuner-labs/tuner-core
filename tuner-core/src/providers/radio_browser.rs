use super::DataProvider;
use crate::models::Station;
use crate::Result;

#[derive(Debug)]
pub struct RadioBrowserProvider;

impl RadioBrowserProvider {
    pub fn new() -> Self {
        Self
    }
}

impl DataProvider for RadioBrowserProvider {
    fn search(&self, _text: &str) -> Result<Vec<Station>> {
        Ok(Vec::new())
    }

    fn name(&self) -> String {
        "Radio Browser 2.0".to_string()
    }
}
