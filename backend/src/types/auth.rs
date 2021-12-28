use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginData {
    /// The email of the user.
    pub email: String,

    /// The password of the user.
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct SessionData {
    /// The user ID for this authorization session.
    pub user_id: u64,

    /// The session token which should be used to log into this session.
    pub token: String, 
}

impl Serialize for SessionData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SessionData", 2)?;

        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("token", &self.token)?;
        state.end()
    }
}
