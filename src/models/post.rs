use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// Placeholder type for holding an image.
type Image = ();

/**
 * A post holds the author and optional content and/or image.
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(skip_serializing_if = "Option::is_none", rename = "_id")]
    pub id: Option<ObjectId>,
    pub author: ObjectId,
    pub content: Option<String>,
    pub image: Option<Image>,
}

impl Post {
    /// Create a new post from an author and content.
    /// This does not save the post to the database!
    pub fn create(author: ObjectId, content: Option<String>, image: Option<Image>) -> Self {
        Post {
            id: None,
            author,
            content,
            image,
        }
    }
}
