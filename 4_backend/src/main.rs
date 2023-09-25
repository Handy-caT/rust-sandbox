use crate::db::db_connect;
use crate::schema::MutationQuery;
use crate::token::get_user_from_request;
use actix_web::error::ErrorInternalServerError;
use actix_web::Error;
use actix_web::{post, web, HttpRequest, HttpServer};
use async_graphql::Schema;
use async_graphql::{EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sea_orm::DatabaseConnection;

mod db;
mod hashing;
mod schema;
mod token;

#[post("/")]
async fn index(
    schema: web::Data<Schema<schema::QueryRoot, MutationQuery, EmptySubscription>>,
    conn: web::Data<DatabaseConnection>,
    http_request: HttpRequest,
    request: GraphQLRequest,
) -> Result<GraphQLResponse, Error> {
    let mut request = request.into_inner();
    let user = get_user_from_request(&http_request, &conn).await;
    if user.is_err() {
        return Err(ErrorInternalServerError("user from token error"));
    }
    let user = user.unwrap();
    println!("user {:?}", user);
    if user.is_some() {
        request = request.data(user.unwrap());
    }
    println!("request {:?}", request);

    Ok(schema.execute(request).await.into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_conn = db_connect().await;
    let schema = Schema::build(schema::QueryRoot, MutationQuery, EmptySubscription)
        .data(db_conn.clone())
        .finish();

    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(db_conn.clone()))
            .service(index)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
