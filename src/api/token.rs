use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};

pub const ENCODING_KEY: EncodingKey = EncodingKey::from_secret("SECRET".as_bytes());
pub const DECODING_KEY: DecodingKey = DecodingKey::from_secret("SECRET".as_bytes());

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for Claims {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = request
            .cookies()
            .get("api-token")
            .map(|cookie| cookie.value());

        let token = match cookie {
            Some(encoded_token) => {
                jsonwebtoken::decode::<Claims>(encoded_token, &DECODING_KEY, &Validation::default())
                    .ok()
                    .map(|token| token.claims)
            }
            None => None,
        };

        match token {
            Some(valid_token) => Outcome::Success(valid_token),
            None => Outcome::Failure((
                Status::Unauthorized,
                "Invalid or missing API token!".to_string(),
            )),
        }
    }
}
