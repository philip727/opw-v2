#[derive(thiserror::Error, Debug)]
pub enum ItemError {
    #[error("No json found at \"{path}\"")]
    NoItemJson {
        path: String
    },
    #[error("A json was found but no items were provided at \"{path}\".")]
    NoItems {
        path: String,
    },
    #[error("The item data provided at \"{path}\" was invalid. ({error})")]
    InvalidData {
        path: String,
        error: String,
    },
}
