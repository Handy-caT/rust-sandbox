use actix_web::{delete, Error, error, get, HttpResponse, post, put, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use utoipa::OpenApi;
use entities::prelude::*;
use entities::{role, user};
use crate::SharedDb;
use role::Model as Role;


pub fn roles_config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_roles)
        .service(show_role)
        .service(create_role)
        .service(update_role)
        .service(delete_role);

}

#[derive(OpenApi)]
#[openapi(components(schemas(Role)),
        paths(
        show_roles,
        show_role,
        create_role,
        update_role,
        delete_role
        )
)]
pub struct RoleDocs;


#[utoipa::path(
    context_path = "/roles",
    responses(
    (status = OK, body = Vec<Model>, description = "List of roles"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
#[get("/")]
async fn show_roles(data: web::Data<SharedDb>) -> Result<HttpResponse, Error> {
    let roles = role::Entity::find()
        .all(&data.db)
        .await;

    if roles.is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }
    let roles = roles.unwrap();

    Ok(HttpResponse::Ok().json(roles))
}

#[utoipa::path(
    context_path = "/roles",
    responses(
    (status = OK, body = Model, description = "Role"),
    (status = NOT_FOUND, description = "Role not found"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    params(
        ("slug" = String, Path, description = "Role slug")
    )
)]
#[get("/{slug}")]
async fn show_role(data: web::Data<SharedDb>, slug: web::Path<String>) -> Result<HttpResponse, Error> {
    let role = role::Entity::find()
        .filter(role::Column::Slug.eq(slug.into_inner()))
        .one(&data.db)
        .await;

    if role.is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }
    let role = role.unwrap();
    if role.is_none() {
        return Err(error::ErrorNotFound("Role not found"));
    }

    Ok(HttpResponse::Ok().json(role))
}

#[utoipa::path(
    context_path = "/roles",
    request_body = Model,
    responses(
    (status = OK, body = String, description = "Role inserted slug"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
#[post("/")]
async fn create_role(data: web::Data<SharedDb>, role: web::Json<role::Model>) -> Result<HttpResponse, Error> {
    let slug = role.slug.clone();
    let active_role: role::ActiveModel = role.into_inner().into_active_model();
    let role = role::Entity::insert(active_role)
        .exec(&data.db)
        .await;

    if role.is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }
    let role = role.unwrap();

    Ok(HttpResponse::Ok().json(slug))
}

#[utoipa::path(
    context_path = "/roles",
    request_body = Model,
    responses(
    (status = OK, body = Model, description = "Role"),
    (status = BAD_REQUEST, description = "Role does not exist"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    params(
        ("slug" = String, Path, description = "Role slug")
    )
)]
#[put("/{slug}")]
async fn update_role(data: web::Data<SharedDb>, slug: web::Path<String>, req_role: web::Json<role::Model>) -> Result<HttpResponse, Error> {
    let slug = slug.into_inner();

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
    let role = role.unwrap();
    if role.slug != slug {
        Err(error::ErrorBadRequest("can't update slug"))?;
    }

    let mut active_role: role::ActiveModel = role.into_active_model();
    active_role.name = Set(req_role.name.clone());
    active_role.permissions = Set(req_role.permissions.clone());

    let role = active_role.update(&data.db).await;
    if role.is_err() {
        return Err(error::ErrorInternalServerError("cannot update role"));
    }
    let role = role.unwrap();

    Ok(HttpResponse::Ok().json(role))
}

#[utoipa::path(
    context_path = "/roles",
    responses(
    (status = OK, body = String, description = "Role deleted slug"),
    (status = BAD_REQUEST, description = "Role does not exist"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    params(
        ("slug" = String, Path, description = "Role slug")
    )
)]
#[delete("/{slug}")]
async fn delete_role(data: web::Data<SharedDb>, slug: web::Path<String>) -> Result<HttpResponse, Error> {
    let slug = slug.into_inner();

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
    let role = role.unwrap();

    let role = role.delete(&data.db).await;
    if role.is_err() {
        Err(error::ErrorInternalServerError("cannot delete role"))?;
    }

    Ok(HttpResponse::Ok().json(slug))
}