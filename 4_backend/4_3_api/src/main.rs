mod db;
mod user;
mod role;

use actix_web::{App, HttpServer, Responder, web};
use futures::StreamExt;
use sea_orm::{DatabaseConnection};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::role::RoleDocs;
use crate::user::UserDocs;


const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[derive(Debug, Clone)]
struct SharedDb {
    db: DatabaseConnection
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = db::db_connect().await;
    let db = SharedDb {
        db
    };

    let users = UserDocs::openapi();
    let roles = RoleDocs::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(web::scope("/users")
                .configure(user::users_config))
            .service(web::scope("/roles")
                .configure(role::roles_config))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/user.json", users.clone()),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/role.json", roles.clone()),
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
