use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use env_logger::Builder;
use host::{reader, writer};
use log::{info, trace, LevelFilter};
mod guest;
mod host;
mod parser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = parser::Args::parse();
    let mut builder = Builder::new();

    let level_filter = match (args.error, args.info, args.trace) {
        (_, _, true) => LevelFilter::Trace,
        (_, true, _) => LevelFilter::Info,
        (true, _, _) => LevelFilter::Error,
        (_, _, _) => LevelFilter::Off,
    };
    builder.filter_level(level_filter).init();

    HttpServer::new(|| App::new().service(read_or_write))
        .bind(("127.0.0.1", 80))?
        .run()
        .await
}

// TODO: Move to its own file "service.rs"

#[get("/{pid}/{address}/{size}/")]
async fn read_or_write(path: web::Path<(i32, usize, usize)>, body: web::Bytes) -> impl Responder {
    let (pid, address, size) = path.into_inner();
    trace!(
        "There was a read or write call with {} {:X} {}",
        pid,
        address,
        size
    );

    // If the body is empty, its a read request.
    if body.is_empty() {
        info!(
            "PID: {} | There has been a read request at {:X} with {} size.",
            pid, address, size
        );

        // Read the specified amounts of bytes at a given address in PID's address space.
        let bytes_read = web::block(move || reader(pid, address as _, size)).await;

        let response = match bytes_read {
            Ok(bytes) => HttpResponse::Ok().body(bytes),
            Err(_) => HttpResponse::BadRequest().body("Invalid read request"),
        };

        response
    } else {
        // Read the bytes inside the request body
        let mut bytes_to_write = body.to_vec();

        info!(
            "PID: {} | There has been a write request at {:X} with contents {:?}.",
            pid, address, bytes_to_write
        );

        // Write the bytes, return how many have been written.
        let bytes_written =
            web::block(move || writer(pid, address as _, &mut bytes_to_write)).await;

        let response = match bytes_written {
            Ok(bytes) => HttpResponse::Ok().body(format!("Successfully written {} bytes", bytes)),
            Err(_) => HttpResponse::BadRequest().body("Invalid write request"),
        };

        response
    }
}

// async fn memflow_read_or_write
// Allow for reading and write of guest operating system through memflow
