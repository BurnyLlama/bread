use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// There are no "profile pictures" in Bread. Instead, each profile has a color.
#[derive(Debug, Serialize, Deserialize)]
pub enum ProfileColor {
    Orange,
    Red,
    Green,
    Blue,
    Grey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub prefers_darkmode: bool,
    pub profile_color: ProfileColor,
}

///A user in Bread.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none", rename = "_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub password: String,
    pub preferences: UserPreferences,
}

impl User {
    /// Create a new user from a name and password hash.
    /// This does not save the user to the database!
    pub fn create(
        name: String,
        password: String,
    ) -> Result<Self, argon2::password_hash::errors::Error> {
        let argon2 = Argon2::default();

        let password_as_buffer = password.as_bytes();
        let password_salt = SaltString::generate(OsRng);
        let hashed_password = match argon2.hash_password(password_as_buffer, &password_salt) {
            Ok(password) => password,
            Err(err) => return Err(err),
        };

        Ok(User {
            id: None,
            name,
            password: hashed_password.to_string(),
            preferences: UserPreferences {
                prefers_darkmode: true,
                profile_color: ProfileColor::Orange,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A test to see if a User is formatted in the way that is wished.
    #[test]
    fn create_user() {
        let user = User::create(
            "Cat_Lover_84".to_string(),
            "This is a hashed password!".to_string(),
        );
        println!("Created the following: {:#?}", user);
    }
}
