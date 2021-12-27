use serde::{Serialize, Serializer, Deserialize, ser::SerializeStruct};

#[derive(Deserialize, Clone)]
pub struct Error {
    /// The message for this error.
    pub message: String,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Error", 1)?;

        state.serialize_field("message", &self.message)?;
        state.end()
    }
}