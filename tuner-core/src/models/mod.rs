pub mod station;
pub use station::Station;

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub station_count: i32,
}
