
#[derive(thiserror::Error, Debug)]
pub enum BiomeError {
    #[error("No biomes were loaded or found in {path}.")]
    NoBiomes {
        path: String
    },
    #[error("No data.json was provided for the {name} biome. ({error})")]
    NoData {
        name: String,
        error: String,
    },
    #[error("The data provided for the {name} biome was invalid. ({error})")]
    InvalidData {
        name: String,
        error: String,
    }
}
