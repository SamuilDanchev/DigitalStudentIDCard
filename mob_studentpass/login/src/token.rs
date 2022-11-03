use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::lib_error::LoginError;

use crate::time;
use crate::time::Expiration;
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims<PAYLOAD> {
    pub payload: Option<PAYLOAD>,
    pub exp: usize,
}

///creates a new token with an expiration date and optionally a generic payload
pub fn create_token<PAYLOAD>(
    secret_key: &str,
    exp: Expiration,
    payload: Option<PAYLOAD>,
) -> Result<String, LoginError>
where
    PAYLOAD: Debug + Serialize,
{
    let exp = time::expiration_date(exp)?;
    let claims = Claims { payload, exp };

    let header = Header::default();

    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )?;

    Ok(token)
}

///checks the token for validity and returned the payload
pub fn verify_token<PAYLOAD>(
    token: String,
    secret_key: &str,
) -> Result<TokenData<Claims<PAYLOAD>>, LoginError>
where
    PAYLOAD: Debug + for<'de> Deserialize<'de>,
{
    let validation = Validation::default();

    let token_data = decode::<Claims<PAYLOAD>>(
        &token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &validation,
    )?;

    Ok(token_data)
}

#[cfg(test)]
mod tests {
    use super::{create_token, verify_token};
    use crate::time::Expiration;
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize)]
    struct Payload {
        pub user: String,
        pub password: String,
    }

    #[test]
    fn verify() {
        let user = Payload {
            user: String::from("TestUser"),
            password: String::from("Test"),
        };

        let token = create_token::<Payload>("secret", Expiration::DAYS(1), Some(user)).unwrap();
        let user = verify_token::<Payload>(token, "secret")
            .unwrap()
            .claims
            .payload
            .unwrap()
            .user;

        assert_eq!(user, "TestUser");
    }
}
