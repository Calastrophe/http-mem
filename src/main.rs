use actix_web::{App, HttpServer};
use clap::Parser;
use env_logger::Builder;
use memflow::prelude::v1::*;
use memflow_win32::prelude::v1::*;
use log::LevelFilter;
use service::{guest_handler, host_handler};
mod guest;
mod host;
mod service;

#[derive(Default, Parser, Debug)]
#[clap(
    author = "Calastrophe",
    version,
    about = "An application to read memory of either the host or guest OS over HTTP."
)]

// NOTE: We don't offer an OS selection because memflow currently only supports Windows target.
pub struct Args {
    #[clap(short, long, default_value_t = 0)]
    pub verbose: u8,
    #[clap(short, long, default_value = "127.0.0.1")]
    pub ip: String,
    // The use of port 80 enforces the use of sudo
    #[clap(short, long, default_value_t = 80)]
    pub port: u16,
    // NOTE: Use memflowup to easily install connectors.
    // curl --proto '=https' --tlsv1.2 -sSf https://sh.memflow.io | sh
    #[clap(short, long, default_value = "kvm")]
    pub connector: String,
    #[clap(short, long, default_value = "win32")]
    pub os: String
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

    HttpServer::new(|| App::new().service(guest_handler).service(host_handler))
        .bind((args.ip, args.port))?
        .run()
        .await
}

fn setup_memflow(args: &Args) -> Result<()> {
    let inventory = Inventory::scan();
    let connector = inventory.builder().os(&args.os).connector(&args.connector).build()?;

    // TODO: Fix this stupid trait bound issue.
    let mut os = Win32Kernel::builder(connector);

    Ok(())
}
