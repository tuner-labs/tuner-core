use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Station {
    pub station_uuid: String,
    pub name: String,
    pub url: String,
    pub country: String,
    pub language: String,
    pub bitrate: i32,
}

impl Station {
    pub fn display_name(&self) -> String {
        format!("{} - {}", self.name, self.country)
    }
}
