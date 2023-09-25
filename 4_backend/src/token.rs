
use actix_web::HttpRequest;
use async_graphql::{Result, Error, Context, Guard};
use time::{Duration, OffsetDateTime};
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use entities::user;
use crate::schema::User;

const TOKEN_VALID_TIME_HOURS: u8 = 24;

pub struct Token(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    user_id: i32,
    expires_in: OffsetDateTime
}

fn get_signing_key() -> Hmac<Sha256> {
    Hmac::new_from_slice(b"some-secret").expect("")
}


impl Token {
    pub fn new(user_id: i32) -> Result<Token, Error> {
        let key = get_signing_key();
        let claims = TokenClaims {
            user_id,
            expires_in: OffsetDateTime::now_utc() + Duration::hours(TOKEN_VALID_TIME_HOURS as i64)
        };

        let token = claims.sign_with_key(&key)
            .expect("");

        Ok(Token(token))
    }

    pub fn validate_token(&self) -> Result<i32> {
        let key = get_signing_key();
        let claims: TokenClaims = self.0
            .verify_with_key(&key)
            .expect("");

        if claims.expires_in < OffsetDateTime::now_utc() {
            return Err(Error::from("not authorized"))
        }
        Ok(claims.user_id)
    }
}

pub async fn get_user_from_request(request: &HttpRequest, conn: &DatabaseConnection) -> Result<Option<User>>{
    let token_option = request.headers().get("Authorization");
    if token_option.is_none() {
        return Ok(None)
    }
    let token_string = token_option.unwrap().to_str()?.to_string();

    let token = Token(token_string);
    let valid = token.validate_token();
    if valid.is_err() {
        return Ok(None)
    }
    let user_id = valid.unwrap();
    println!("user id {}", user_id);

    let user = user::Entity::find_by_id(user_id)
        .one(conn)
        .await
        .map_err(|_| Error::from("user not found"))?;
    if user.is_none() {
        return Err(Error::from("user not found"));
    }
    let user = user.unwrap();
    println!("user {:?}", user);

    Ok(Some(User {
        id: user.id,
        name: user.name
    }))
}

pub struct ValidTokenGuard;

impl ValidTokenGuard {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Guard for ValidTokenGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<User>().is_some() {
            Ok(())
        } else {
            Err(Error::from("Not authorized"))
        }
    }
}