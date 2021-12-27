use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Deserialize, Clone)]
pub struct Error {
    /// The message for this error.
    pub message: String,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 1)?;

        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

#[derive(Deserialize, Clone)]
pub struct Success {
    /// The success message.
    pub message: String,

    /// The ID of whatever object was created, if applicable.
    pub id: Option<u64>,
}

impl Serialize for Success {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Success", 2)?;

        state.serialize_field("message", &self.message)?;
        state.serialize_field("id", &self.id)?;
        state.end()
    }
}
