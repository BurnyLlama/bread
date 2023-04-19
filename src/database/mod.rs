use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::DeleteResult,
    sync::{Client, Collection},
};
use std::{env, fmt::Display};

use crate::models::{post::Post, user::User};

/// This holds a database and makes shortcuts for the respective collections.
pub struct DatabaseHandler {
    // db: Database,
    users: Collection<User>,
    posts: Collection<Post>,
}

/// This function converts an error to a string so that errors easily can be used in a Result<T, String> no matter what caused the error.
fn err_to_string<E: Display>(err: E) -> String {
    err.to_string()
}

impl DatabaseHandler {
    /// Create a connection to a database.
    /// This requires a file called ".env" with the environment variable "MONGO_URI" in it.
    pub fn create_connection() -> Result<Self, String> {
        dotenv().map_err(err_to_string)?;
        let uri = env::var("MONGO_URI").map_err(err_to_string)?;
        let client = Client::with_uri_str(uri).map_err(err_to_string)?;
        let db = client.database("bread");
        let users = db.collection::<User>("users");
        let posts = db.collection::<Post>("posts");
        Ok(Self { users, posts })
    }

    /*
     * USERS
     */

    /// Saves a user to the database.
    /// Returns the id of the created user as an option.
    pub fn save_user(&self, user: &User) -> Result<Option<ObjectId>, String> {
        let result = self.users.insert_one(user, None).map_err(err_to_string)?;
        Ok(result.inserted_id.as_object_id())
    }

    /// Get a user from the database via its id.
    pub fn find_user_by_id(&self, id: &ObjectId) -> Result<Option<User>, String> {
        self.users
            .find_one(doc! { "_id": id }, None)
            .map_err(err_to_string)
    }

    /// Get a user from the database via its name.
    pub fn find_user_by_name(&self, name: &str) -> Result<Option<User>, String> {
        self.users
            .find_one(doc! { "name": name }, None)
            .map_err(err_to_string)
    }

    /// Delete a user from the database via its id.
    pub fn delete_user(&self, id: &ObjectId) -> Result<DeleteResult, String> {
        self.users
            .delete_one(doc! { "_id": id }, None)
            .map_err(err_to_string)
    }

    /*
     * POSTS
     */

    /// Saves a user to the database.
    /// Returns the id of the created post as an option.
    pub fn save_post(&self, post: &Post) -> Result<Option<ObjectId>, String> {
        let result = self.posts.insert_one(post, None).map_err(err_to_string)?;
        Ok(result.inserted_id.as_object_id())
    }

    /// Get a user from the database via its id.
    pub fn find_post_by_id(&self, id: &ObjectId) -> Result<Option<Post>, String> {
        self.posts
            .find_one(doc! { "_id": id }, None)
            .map_err(err_to_string)
    }

    /// Delete a post from the database. Only the author of the post is allowed to delete it.
    pub fn delete_post(
        &self,
        user_deleting_post: &ObjectId,
        post_id: &ObjectId,
    ) -> Result<DeleteResult, String> {
        let post = self.find_post_by_id(post_id).map_err(err_to_string)?;

        if post.is_none() {
            return Err("Post not found!".to_string());
        }

        let post = post.unwrap();

        // Make sure the author is the one trying to delete the post.
        if &post.author != user_deleting_post {
            return Err("Deleter is not author of post!".to_string());
        }

        self.posts
            .delete_one(doc! { "_id": post.id }, None)
            .map_err(err_to_string)
    }

    /// Fetch a random post from the database.
    pub fn find_random_post(&self) -> Result<Post, String> {
        // This aggregates one random post.
        let aggregation = self
            .posts
            .aggregate([doc! { "$sample": { "size": 1 } }], None);

        // Check if the aggregation returned a result or an error.
        // If a result, handle it to read the result and turn it into a Post.
        // If an error, wrap it into a string.
        match aggregation {
            Ok(mut cursor) => {
                let result = match cursor.next() {
                    Some(result) => result,
                    None => return Err("No results were found!".to_string()),
                };

                match result {
                    // Create a Post from the document data.
                    Ok(document_data) => {
                        mongodb::bson::from_document(document_data).map_err(err_to_string)
                    }
                    // Turn the error into a string.
                    Err(err) => Err(err_to_string(err)),
                }
            }
            Err(err) => Err(err_to_string(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This tests checks that it is possible to save a user to the database.
    #[test]
    fn create_and_save_user_and_post() {
        // Try to connect to the database, panic if it fails.
        let db_handler = match DatabaseHandler::create_connection() {
            Ok(handler) => handler,
            Err(err) => panic!("Could not connect to the database! Error: {}", err),
        };

        // Create a dummy user for the test.
        let user = match User::create("Foo".to_string(), "Bar".to_string()) {
            Ok(user) => user,
            Err(err) => panic!("Could not create user! {:?}", err),
        };

        // Try to save the user, if it fails, panic.
        let user_id = match db_handler.save_user(&user) {
            Ok(user_id) => user_id.expect("Getting ObjectId failed!"),
            Err(err) => panic!("Could not save the user! Error: {}", err),
        };

        // Create a dummy post for the test. (It will not use a real user!)
        let post = Post::create(user_id, Some("Foo".to_string()), Some(()));

        // Try to save the user, if it fails, panic.
        match db_handler.save_post(&post) {
            Ok(_) => println!("Success!"),
            Err(err) => panic!("Could not save the user! Error: {}", err),
        };
    }

    /// Make sure deletion of user and post works.
    #[test]
    fn create_save_and_delete_user_and_post() {
        // Try to connect to the database, panic if it fails.
        let db_handler = match DatabaseHandler::create_connection() {
            Ok(handler) => handler,
            Err(err) => panic!("Could not connect to the database! Error: {}", err),
        };

        // Create a dummy user for the test.
        let user = match User::create("Foo".to_string(), "Bar".to_string()) {
            Ok(user) => user,
            Err(err) => panic!("Could not create user! {:?}", err),
        };

        // Try to save the user, if it fails, panic.
        let user_id = match db_handler.save_user(&user) {
            Ok(user_id) => user_id.expect("Getting ObjectId failed!"),
            Err(err) => panic!("Could not save the user! Error: {}", err),
        };

        // Create a dummy post for the test. (It will not use a real user!)
        let post = Post::create(user_id, Some("Foo".to_string()), Some(()));

        // Try to save the user, if it fails, panic.
        let post_id = match db_handler.save_post(&post) {
            Ok(id) => id.expect("Could not get the post's id!"),
            Err(err) => panic!("Could not save the user! Error: {}", err),
        };

        match db_handler.delete_post(&user_id, &post_id) {
            Ok(_) => println!("Successfully deleted post!"),
            Err(err) => println!("Could not delete post! {:?}", err),
        }

        match db_handler.delete_user(&user_id) {
            Ok(_) => println!("Successfully deleted user!"),
            Err(err) => println!("Could not delete post! {:?}", err),
        }
    }

    /// Make sure it is possible to find a random post.
    #[test]
    fn find_random_post() {
        let db_handler = match DatabaseHandler::create_connection() {
            Ok(handler) => handler,
            Err(err) => panic!("Could not connect to the database! Error: {}", err),
        };

        match db_handler.find_random_post() {
            Ok(post) => println!("Success! Found post: {:?}", post),
            Err(err) => panic!("{}", err),
        };
    }
}
