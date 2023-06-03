use actix_web::{web, App, HttpServer};
use clap::Parser;
use env_logger::Builder;
use log::{info, LevelFilter};
use memflow::prelude::v1::*;
use service::{guest_handler, host_handler};
use std::sync::Mutex;
mod guest;
mod host;
mod service;

// NOTE: Use memflowup to easily install connectors.
// curl --proto '=https' --tlsv1.2 -sSf https://sh.memflow.io | sh

#[derive(Default, Parser, Debug)]
#[clap(
    author = "Calastrophe",
    version,
    about = "An application to read memory of either the host or guest OS over HTTP."
)]

pub struct Args {
    #[clap(short, long, default_value_t = 0)]
    pub verbose: u8,
    #[clap(short, long, default_value = "127.0.0.1")]
    pub ip: String,
    // The use of port 80 enforces the use of sudo
    #[clap(short, long, default_value_t = 80)]
    pub port: u16,
    #[clap(short, long, default_value_t = true)]
    pub memflow: bool,
    #[clap(short, long, default_value = "kvm")]
    pub connector: String,
    #[clap(short, long, default_value = "win32")]
    pub os: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut builder = Builder::new();

    let level_filter = match args.verbose {
        0 => LevelFilter::Off,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Error,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    builder.filter_level(level_filter).init();

    match args.memflow {
        true => {
            let inventory = Inventory::scan();
            let connector = inventory
                .create_connector(&args.connector, None, None)
                .expect("connector init failed");
            let os = inventory
                .create_os(&args.os, Some(connector), None)
                .expect("os init failed");
            let shared_os = web::Data::new(Mutex::new(os));
            HttpServer::new(move || {
                App::new()
                    .service(guest_handler)
                    .service(host_handler)
                    .app_data(shared_os.clone())
            })
            .bind((args.ip, args.port))?
            .run()
            .await
        }
        _ => {
            HttpServer::new(|| App::new().service(host_handler))
                .bind((args.ip, args.port))?
                .run()
                .await
        }
    }
}
