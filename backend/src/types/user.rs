use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Deserialize, Clone)]
pub struct User {
    /// The user's unique ID.
    pub id: u64,

    /// The username of this user.
    pub name: String,

    /// The discriminator of this user.
    pub discriminator: u16,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("User", 3)?;

        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("discriminator", &self.discriminator)?;
        state.end()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateUserData {
    /// The username of this user.
    pub name: String,

    /// The email of this user.
    pub email: String,

    /// The password of this user.
    pub password: String,
}
