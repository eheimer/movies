#[derive(Debug, Clone)]
pub enum Entry {
  Series { id: i32, name: String},
  Episode { id: i32, name: String, location: String},
}