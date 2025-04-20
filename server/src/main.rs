use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use compiler::{compile, compile_to_arduino, CompilerError};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct CompileRequest {
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompileResponse {
    output: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

async fn compile_ir(req: web::Json<CompileRequest>) -> impl Responder {
    match compile(req.code.clone()) {
        Ok(output) => HttpResponse::Ok().json(CompileResponse { output }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

async fn compile_arduino(req: web::Json<CompileRequest>) -> impl Responder {
    match compile_to_arduino(req.code.clone()) {
        Ok(output) => HttpResponse::Ok().json(CompileResponse { output }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_address = format!("{}:{}", host, port);

    println!("Starting server at {}", bind_address);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(
                web::scope("/api")
                    .route("/compile", web::post().to(compile_ir))
                    .route("/compile/arduino", web::post().to(compile_arduino)),
            )
    })
    .bind(bind_address)?
    .run()
    .await
} 