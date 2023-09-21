use actix_web::{delete, Error, error, get, HttpResponse, post, put, web};
use actix_web::web::Json;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, IntoActiveModel, JoinType, ModelTrait, QueryFilter, QuerySelect};
use sea_orm::ActiveValue::Set;
use utoipa::{OpenApi, ToSchema};
use serde::{Deserialize, Serialize};
use entities::{user, users_roles, role};
use crate::{SharedDb};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct UserRequestModel {
    #[schema(example = "John Doe")]
    name: String,
    #[schema(example = "admin")]
    role: String,
}

#[derive(FromQueryResult, Debug, Serialize, Deserialize, ToSchema)]
struct UserAndRole {
    #[schema(example = 1)]
    id: i32,
    #[schema(example = "John Doe")]
    name: String,
    #[schema(example = "admin")]
    role_slug: String,
}

#[derive(OpenApi)]
#[openapi(
            paths(
                show_users,
                show_user,
                create_user,
                assign_role,
                unassign_role,
                update_user,
                delete_user
            ),
            components(schemas(UserAndRole, UserRequestModel, user::Model))
)]
pub struct UserDocs;

pub fn users_config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_users)
        .service(show_user)
        .service(create_user)
        .service(assign_role)
        .service(unassign_role)
        .service(update_user)
        .service(delete_user);
}

#[utoipa::path(
    context_path = "/users",
    responses(
    (status = OK, body = Vec<UserAndRole>, description = "List of users and their roles"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
    )
)]
#[get("/")]
async fn show_users(data: web::Data<SharedDb>) -> Result<Json<Vec<UserAndRole>>, Error> {
    let user_select = user::Entity::find()
        .column(users_roles::Column::RoleSlug)
        .join_rev(
            JoinType::LeftJoin,
            users_roles::Entity::belongs_to(user::Entity)
                .from(users_roles::Column::UserId)
                .to(user::Column::Id)
                .into()
        )
        .into_model::<UserAndRole>()
        .all(&data.db)
        .await;

    if user_select.is_err() {
        return Err(error::ErrorInternalServerError("cannot select users"));
    };
    let users = user_select.unwrap();

    Ok(Json(users))
}

#[utoipa::path(
    context_path = "/users",
    responses(
    (status = OK, body = Vec<UserAndRole>, description = "List of users and their roles"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
    (status = NOT_FOUND, description = "User not found"),
    ),
    params(
        ("id" = i32, Path, description = "User id")
    )
)]
#[get("/{id}")]
async fn show_user(data: web::Data<SharedDb>, id: web::Path<i32>) -> Result<Json<Vec<UserAndRole>>, Error> {
    let user_select = user::Entity::find()
        .column(users_roles::Column::RoleSlug)
        .filter(user::Column::Id.eq(id.into_inner()))
        .join_rev(
            JoinType::LeftJoin,
            users_roles::Entity::belongs_to(user::Entity)
                .from(users_roles::Column::UserId)
                .to(user::Column::Id)
                .into()
        )
        .into_model::<UserAndRole>()
        .all(&data.db)
        .await;

    if user_select.is_err() {
        return Err(error::ErrorInternalServerError("cannot select users"));
    };
    let users = user_select.unwrap();
    if users.len() == 0 {
        return Err(error::ErrorNotFound("user not found"));
    }

    Ok(Json(users))
}

#[utoipa::path(
    context_path = "/users",
    request_body = UserRequestModel,
    responses(
    (status = OK, body = UserAndRole, description = "User and his role"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
    (status = BAD_REQUEST, description = "User already exists"),
    )
)]
#[post("/")]
async fn create_user(data: web::Data<SharedDb>, user: Json<UserRequestModel>) -> Result<Json<UserAndRole>, Error> {
    let role_slug = user.role.clone();
    let name = user.name.clone();

    let user = user::ActiveModel {
        name: Set(name.clone()),
        ..Default::default()
    };

    let mut user_role = users_roles::ActiveModel {
        role_slug: Set(role_slug),
        ..Default::default()
    };
    let user = user::Entity::insert(user)
        .exec(&data.db)
        .await;
    if user.is_err() {
        return Err(error::ErrorBadRequest("user already exists"));
    }
    let id = user.unwrap().last_insert_id;
    user_role.user_id = Set(id);
    let res = user_role.insert(&data.db).await;
    if res.is_err() {
        return Err(error::ErrorBadRequest("user already has this role"));
    }
    let res = res.unwrap();

    let res = UserAndRole {
        id,
        name,
        role_slug: res.role_slug,
    };

    Ok(Json(res))
}

#[utoipa::path(
    responses(
    (status = OK, body = String, description = "Role slug"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
    (status = BAD_REQUEST, description = "Role does not exist or is already assigned"),
    ),
    params(
        ("id" = i32, Path, description = "User id"),
        ("role_slug" = String, Path, description = "Role slug")
    )
)]
#[post("/{id}/assign/{role_slug}")]
async fn assign_role(data: web::Data<SharedDb>, params: web::Path<(i32, String)>) -> Result<HttpResponse, Error> {
    let slug = params.1.clone();
    let id = params.0;

    let role = role::Entity::find()
        .filter(role::Column::Slug.eq(&slug))
        .one(&data.db)
        .await;

    if role.is_err() {
        Err(error::ErrorBadRequest("role does not exist"))?;
    }
    let role = role.unwrap();
    if role.is_none() {
        Err(error::ErrorBadRequest("role does not exist"))?;
    }

    let user_role = users_roles::Entity::find()
        .filter(users_roles::Column::UserId.eq(id))
        .filter(users_roles::Column::RoleSlug.eq(&slug))
        .one(&data.db)
        .await;

    if user_role.is_ok() {
        let user_role = user_role.unwrap();
        if user_role.is_some() {
            Err(error::ErrorBadRequest("Role is already assigned"))
        } else {
            let user_role = users_roles::ActiveModel {
                user_id: Set(id),
                role_slug: Set(slug.clone()),
                ..Default::default()
            };

            let res = user_role.insert(&data.db).await;
            if res.is_err() {
                return Err(error::ErrorInternalServerError("Database internal error"));
            };

            Ok(HttpResponse::Ok().json(slug))
        }
    } else {
        Err(error::ErrorInternalServerError("Database internal error"))
    }
}

#[utoipa::path(
    responses(
    (status = OK, body = String, description = "Role slug"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
    (status = BAD_REQUEST, description = "Role is not assigned"),
    ),
    params(
        ("id" = i32, Path, description = "User id"),
        ("role_slug" = String, Path, description = "Role slug")
    )
)]
#[post("/{id}/unassign/{role_slug}")]
async fn unassign_role(data: web::Data<SharedDb>, params: web::Path<(i32, String)>) -> Result<HttpResponse, Error> {
    let slug = params.1.clone();
    let id = params.0;

    let user_role = users_roles::Entity::find()
        .filter(users_roles::Column::UserId.eq(id))
        .filter(users_roles::Column::RoleSlug.eq(&slug))
        .one(&data.db)
        .await;

    if user_role.is_err() {
        Err(error::ErrorBadRequest("Role is not assigned"))
    } else {
        let user_role = user_role.unwrap();
        if user_role.is_none() {
            Err(error::ErrorBadRequest("Role is not assigned"))
        } else {
            let user_role = user_role.unwrap();
            let user_role = user_role.delete(&data.db).await;
            if user_role.is_err() {
                Err(error::ErrorInternalServerError("Database internal error"))
            } else {
                Ok(HttpResponse::Ok().json(slug))
            }
        }
    }
}

#[utoipa::path(
    context_path = "/users",
    request_body = Model,
    responses(
    (status = OK, body = UserAndRole, description = "User and his role"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
    (status = BAD_REQUEST, description = "User does not exist or bad id"),
    )
)]
#[put("/{id}")]
async fn update_user(data: web::Data<SharedDb>, id: web::Path<i32>, user: Json<user::Model>) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let name = user.name.clone();
    if id != user.id {
        Err(error::ErrorBadRequest("bad id"))?;
    }

    let user = user::Entity::find_by_id(id)
        .one(&data.db)
        .await;

    if user.is_err() {
        Err(error::ErrorBadRequest("user does not exist"))?;
    }
    let user = user.unwrap();
    if user.is_none() {
        Err(error::ErrorBadRequest("user does not exist"))?;
    }
    let mut user = user.unwrap();

    let mut user: user::ActiveModel = user.into();
    user.name = Set(name);

    let user = user.update(&data.db).await;
    if user.is_err() {
        Err(error::ErrorInternalServerError("cannot update user"))?;
    }
    let user = user.unwrap();

    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    context_path = "/users",
    responses(
    (status = OK, body = i32, description = "User id"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
    (status = BAD_REQUEST, description = "User does not exist"),
    )
)]
#[delete("/{id}")]
async fn delete_user(data: web::Data<SharedDb>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let id = id.into_inner();

    let user = user::Entity::find_by_id(id)
        .one(&data.db)
        .await;

    if user.is_err() {
        Err(error::ErrorBadRequest("user does not exist"))?;
    }
    let user = user.unwrap();
    if user.is_none() {
        Err(error::ErrorBadRequest("user does not exist"))?;
    }
    let user = user.unwrap();

    let user = user.delete(&data.db).await;
    if user.is_err() {
        Err(error::ErrorInternalServerError("cannot delete user"))?;
    }

    Ok(HttpResponse::Ok().json(id))
}