use crate::models::Station;
use crate::Result;

pub trait Storage: Send + Sync {
    fn get_favorites(&self) -> Result<Vec<Station>>;
    fn add_favorite(&self, station: &Station) -> Result<()>;
}

#[derive(Default)]
pub struct MemoryStorage {
    favorites: Vec<Station>,
}

impl Storage for MemoryStorage {
    fn get_favorites(&self) -> Result<Vec<Station>> {
        Ok(self.favorites.clone())
    }

    fn add_favorite(&self, _station: &Station) -> Result<()> {
        Ok(())
    }
}
