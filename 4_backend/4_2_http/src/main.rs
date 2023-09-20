mod request_processor;
mod db;

use actix_web::{App, error, Error, HttpResponse, HttpServer, post, Responder, web};
use futures::StreamExt;
use sea_orm::{DatabaseConnection};
use serde_json::Value;

use crate::request_processor::RequestProcessor;


const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("/")]
async fn index_manual(mut payload: web::Payload, data: web::Data<SharedDb>) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let root: Value = serde_json::from_slice(&body)?;
    println!("{:?}", root);
    let request_processor = RequestProcessor::new(data.db.clone());
    let command_type = request_processor.process_request(root).await?;

    Ok(HttpResponse::Ok().json(command_type)) // <- send response
}

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



    HttpServer::new(move || {
        App::new()
            .service(index_manual)
            .app_data(web::Data::new(db.clone()))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
