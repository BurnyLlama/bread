use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ProfileColor {
    Orange,
    Red,
    Green,
    Blue,
    Grey
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub prefers_darkmode: bool,
    pub profile_color: ProfileColor
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none", rename = "_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub password: String,
    pub preferences: UserPreferences,
}

impl User {
    pub fn create(name: String, password: String) -> Self {
        User {
            name: name,
            password: password,
            id: None,
            preferences: UserPreferences {
                prefers_darkmode: true,
                profile_color: ProfileColor::Orange
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user() {
        let user = User::create("Cat_Lover_84".to_string(), "This is a hashed password!".to_string());
        println!("Created the following: {:#?}", user);
    }
}