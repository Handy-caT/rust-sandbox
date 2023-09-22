use actix_web::{HttpServer, post, web};
use actix_web::http::Error;
use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod schema;

#[post("/")]
async fn index(schema: web::Data<Schema<schema::QueryRoot, EmptyMutation, EmptySubscription>>,
               request: GraphQLRequest) -> Result<GraphQLResponse, Error> {
    Ok(schema.execute(request.into_inner()).await.into())
}

#[actix_web::main]
async fn main() {
    let schema = Schema::build(schema::QueryRoot, EmptyMutation, EmptySubscription)
        .finish();

    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(schema.clone())
            .service(index)
    });
}
