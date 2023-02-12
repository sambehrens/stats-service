#[derive(serde::Serialize)]
pub struct Error {
    reason: String,
}

impl Error {
    pub fn new(reason: &str) -> Self {
        Self {
            reason: reason.to_string(),
        }
    }
}
