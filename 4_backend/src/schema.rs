use async_graphql::{Context, Error, Object};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use async_graphql::Result;
use sea_orm::ActiveValue::Set;
use secrecy::SecretString;
use entities::user;
use entities::prelude::*;
use crate::hashing::{hash_password, validate_password_hash};
use crate::token::{Token, ValidTokenGuard};

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
}


#[Object]
impl User {
    pub async fn id(&self) -> i32 {
        self.id
    }

    pub async fn name(&self) -> &Option<String> {
        &self.name
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    #[graphql(guard = "ValidTokenGuard")]
    pub async fn user<'ctx>(&self, ctx: &Context<'ctx>, user_id: Option<i32>) -> Result<Vec<User>> {
        let conn = ctx.data_unchecked::<DatabaseConnection>();
        if user_id.is_some() {
            let user = user::Entity::find_by_id(user_id.unwrap()).one(conn).await.unwrap();
            let user = user.unwrap();
            let user = User {
                id: user.id,
                name: user.name,
            };
            Ok(vec![user])
        } else {
            let users = user::Entity::find()
                .all(conn)
                .await
                .unwrap();
            let users = users.iter().map(|user| {
                User {
                    id: user.id,
                    name: user.name.clone(),
                }
            }).collect();

            Ok(users)
        }

    }
}

pub struct MutationQuery;

#[Object]
impl MutationQuery {
    pub async fn create_user<'ctx>(&self, ctx: &Context<'ctx>, name: String, password: String) -> Result<User> {
        let conn = ctx.data_unchecked::<DatabaseConnection>();
        let password_hash = hash_password(password);
        let password_hash = password_hash.unwrap();

        let user = user::ActiveModel {
            name: Set(Some(name)),
            password_hash: Set(Some(password_hash)),
            ..Default::default()
        };

        let user = user.insert(conn)
            .await
            .unwrap();

        Ok(
            User {
                id: user.id,
                name: user.name,
            }
        )
    }

    pub async fn login<'ctx>(&self, ctx: &Context<'ctx>, name: String, password: String) -> Result<String> {
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        let user = user::Entity::find()
            .filter(user::Column::Name.eq(name))
            .one(conn)
            .await
            .unwrap()
            .unwrap();

        let hash = user.password_hash.unwrap();
        let valid = validate_password_hash(password, hash);
        if !valid {
            return Err(Error::new("Authentication failed"))
        }
        let token = Token::new(user.id);
        if token.is_err() {
            return Err(Error::new("Authentication failed"))
        }
        let token = token.unwrap();

        Ok(token.0)
    }
}