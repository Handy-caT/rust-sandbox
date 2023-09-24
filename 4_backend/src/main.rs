use actix_web::{HttpServer, post, web};
use actix_web::http::Error;
use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::db::db_connect;
use crate::schema::MutationQuery;

mod schema;
mod db;
mod hashing;

#[post("/")]
async fn index(schema: web::Data<Schema<schema::QueryRoot, MutationQuery, EmptySubscription>>,
               request: GraphQLRequest) -> Result<GraphQLResponse, Error> {
    Ok(schema.execute(request.into_inner()).await.into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_conn = db_connect().await;
    let schema = Schema::build(schema::QueryRoot, MutationQuery, EmptySubscription)
        .data(db_conn)
        .finish();

    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(index)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
