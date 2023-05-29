use crate::host::{reader, writer};
use actix_web::{get, web, HttpResponse, Responder};
use log::{info, trace};

// Linux implementation for host read/write requests

#[get("/host/{pid}/{address}/{size}")]
async fn host_handler(path: web::Path<(i32, usize, usize)>, body: web::Bytes) -> impl Responder {
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

// Implementation for guest read/write requests.
// NOTE: memflow only supports windows targets, but has many connectors.

#[get("/guest/{pid}/{address}/{size}")]
async fn guest_handler(path: web::Path<(i32, usize, usize)>, body: web::Bytes) -> impl Responder {
    format!("unimplemented")
}
