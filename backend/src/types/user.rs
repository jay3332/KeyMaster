use crate::json::JsonResponse;
use crate::types::Error;
use bitflags::bitflags;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Deserialize, Clone)]
pub struct User {
    /// The user's unique ID.
    pub id: u64,

    /// The username of this user.
    pub name: String,

    /// The discriminator of this user.
    pub discriminator: u16,

    /// The email of this user. This is only available through the `/users/me` route.
    pub email: Option<String>,

    /// A bitmask representing the permissions this user has. This is only available through the `/users/me` route.
    pub permissions: Option<UserPermissionFlags>,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("User", 5)?;

        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("discriminator", &self.discriminator)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("permissions", &self.permissions)?;
        state.end()
    }
}

bitflags! {
    pub struct UserPermissionFlags: u64 {
        const OWNER = 1 << 0;
        const ADD_QUOTES = 1 << 1;
    }
}

serde_bitflags!(UserPermissionFlags);

impl UserPermissionFlags {
    pub fn has_permission(&self, permission: UserPermissionFlags) -> bool {
        self.contains(permission) || self.contains(UserPermissionFlags::OWNER)
    }

    pub fn has_any_permission(&self, permissions: &[UserPermissionFlags]) -> bool {
        self.contains(UserPermissionFlags::OWNER)
            || permissions
                .iter()
                .any(|permission| self.contains(*permission))
    }

    pub fn has_all_permissions(&self, permissions: &[UserPermissionFlags]) -> bool {
        self.contains(UserPermissionFlags::OWNER)
            || permissions
                .iter()
                .all(|permission| self.contains(*permission))
    }

    fn err() -> Result<!, JsonResponse<Error>> {
        Err(JsonResponse::new(
            403,
            Error {
                message: "You do not have permission to perform this action".to_string(),
            },
        ))
    }

    pub fn expect_permission(
        &self,
        permission: UserPermissionFlags,
    ) -> Result<(), JsonResponse<Error>> {
        if !self.has_permission(permission) {
            Self::err()?
        }
        Ok(())
    }

    pub fn expect_any_permission(
        &self,
        permissions: &[UserPermissionFlags],
    ) -> Result<(), JsonResponse<Error>> {
        if !self.has_any_permission(permissions) {
            Self::err()?
        }
        Ok(())
    }

    pub fn expect_all_permissions(
        &self,
        permissions: &[UserPermissionFlags],
    ) -> Result<(), JsonResponse<Error>> {
        if !self.has_all_permissions(permissions) {
            Self::err()?
        }
        Ok(())
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
