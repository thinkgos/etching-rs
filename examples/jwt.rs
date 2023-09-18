use jsonwebtoken::{EncodingKey, Header, /*decode, encode, Algorithm, DecodingKey, Validation */};
use serde::{Deserialize, Serialize};
use std::error;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let token = jsonwebtoken::encode(
        &Header::default(),
        &Claims {
            aud: "1".to_owned(),
            exp: 1111,
            iat: 1111,
            iss: "1".to_owned(),
            nbf: 1111,
            sub: "1".to_owned(),
        },
        &EncodingKey::from_secret("secret".as_ref()),
    )?;
    println!("{}", token);

    Ok(())
}
