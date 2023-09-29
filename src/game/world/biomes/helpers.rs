use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomeData {
    pub id: String,
    pub texture_location: String,
}
