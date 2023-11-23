#[derive(thiserror::Error, Debug)]
pub enum WorldSelectionError {
    #[error("No worlds.")]
    NoneFound,
    #[error("No settings were found for the world \"{name}\".")]
    NoSettings {
        name: String
    },
    #[error("The data provided for the world \"{name}\" was invalid. ({error}).")]
    InvalidData {
        name: String,
        error: String,
    },
    #[error("Failed to decode the world data. (\"{name}\") ({error})")]
    FailedDecode {
        name: String,
        error: String,
    }
}
