use std::{env, fmt::Display};
use dotenv::dotenv;
use mongodb::{bson::{extjson::de::Error},sync::{Client, Collection, Database}, results::InsertOneResult};

use crate::models::user::User;

pub struct DatabaseHandler {
    db: Database,
    users: Collection<User>
}

fn err_to_string<E: Display>(err: E) -> String {
    err.to_string()
}

impl DatabaseHandler {
    pub fn create_connection() -> Result<Self, String> {
        dotenv().map_err(err_to_string)?;
        let uri = env::var("MONGO_URI").map_err(err_to_string)?;
        let client = Client::with_uri_str(uri).map_err(err_to_string)?;
        let db = client.database("bread");
        let users = db.collection::<User>("users");
        Ok(Self { db, users })
    }

    pub fn create_user(&self, name: String, password: String) -> Result<InsertOneResult, String> {
        let new_user = User::create(name, password);
        self
            .users
            .insert_one(new_user, None)
            .map_err(err_to_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_save_user() {
        let db_handler = match DatabaseHandler::create_connection() {
            Ok(handler) => handler,
            Err(err) => panic!("{}", err)
        };

        let created_user = db_handler.create_user("Foo".to_string(), "Bar".to_string());
        println!("Created: {:#?}", created_user);
    }
}