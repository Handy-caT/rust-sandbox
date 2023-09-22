use std::fmt::Error;
use async_graphql::{Context, Object};
use sea_orm::{DatabaseConnection, EntityTrait};
use entities::user;

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
    pub async fn user<'ctx>(&self, ctx: &Context<'ctx>, user_id: Option<i32>) -> Result<User, Error> {
        let conn = ctx.data_unchecked::<DatabaseConnection>();
        let user = user::Entity::find_by_id(user_id.unwrap()).one(conn).await.unwrap();
        let user = user.unwrap();
        Ok(User {
            id: user.id,
            name: user.name,
        })
    }
}