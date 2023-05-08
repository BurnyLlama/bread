use argon2::{Config, ThreadMode, Variant, Version};
use mongodb::bson::oid::ObjectId;
use password_hash::rand_core::OsRng;
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
    pub fn create(name: String, password: String) -> Result<Self, String> {
        let hashed_password = Self::hash_password(&password)?;

        Ok(User {
            id: None,
            name,
            password: hashed_password,
            preferences: UserPreferences {
                prefers_darkmode: true,
                profile_color: ProfileColor::Orange,
            },
        })
    }

    pub fn hash_password(password: &str) -> Result<String, String> {
        let argon2_config = Config {
            variant: Variant::Argon2id,
            version: Version::Version13,
            mem_cost: 65536,
            time_cost: 10,
            lanes: 4,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: 32,
        };

        // Create a random salt for the hash.
        let password_salt = password_hash::SaltString::generate(OsRng)
            .as_str()
            .to_owned();

        let hashed_password = argon2::hash_encoded(
            password.as_bytes(),
            password_salt.as_bytes(),
            &argon2_config,
        )
        .map_err(|err| err.to_string());

        hashed_password
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
