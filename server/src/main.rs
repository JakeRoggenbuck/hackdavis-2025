use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use compiler::{compile, compile_to_arduino, CompilerError};
use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;
use std::path::Path;
use tempfile::tempdir;

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

#[derive(Debug, Serialize, Deserialize)]
struct UploadRequest {
    code: String,
    port: String,
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

async fn upload_arduino(req: web::Json<UploadRequest>) -> impl Responder {
    // Create a temporary directory for the sketch
    let temp_dir = match tempdir() {
        Ok(dir) => dir,
        Err(e) => return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Failed to create temporary directory: {}", e),
        }),
    };

    // Create the sketch file
    let sketch_path = temp_dir.path().join("sketch.ino");
    if let Err(e) = std::fs::write(&sketch_path, &req.code) {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Failed to write sketch file: {}", e),
        });
    }

    // Compile the sketch
    let compile_output = Command::new("arduino-cli")
        .arg("compile")
        .arg("--fqbn")
        .arg("arduino:avr:uno")
        .arg(temp_dir.path())
        .output();

    match compile_output {
        Ok(output) if !output.status.success() => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Failed to compile: {}", e),
            });
        }
        _ => {}
    }

    // Upload the sketch
    let upload_output = Command::new("arduino-cli")
        .arg("upload")
        .arg("-p")
        .arg(&req.port)
        .arg("--fqbn")
        .arg("arduino:avr:uno")
        .arg(temp_dir.path())
        .output();

    match upload_output {
        Ok(output) if output.status.success() => {
            HttpResponse::Ok().json(CompileResponse {
                output: "Upload successful".to_string(),
            })
        }
        Ok(output) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Failed to upload: {}", e),
            })
        }
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
                    .route("/compile/arduino", web::post().to(compile_arduino))
                    .route("/upload/arduino", web::post().to(upload_arduino)),
            )
    })
    .bind(bind_address)?
    .run()
    .await
} 