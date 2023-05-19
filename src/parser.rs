use clap::Parser;

#[derive(Default, Parser, Debug)]
#[clap(
    author = "Calastrophe",
    version,
    about = "An application to read memory of either the host or guest OS over HTTP."
)]
pub struct Args {
    /// Uses memflow to process memory read and writes to Guest OS.
    #[clap(short, long, default_value_t = false)]
    pub memflow: bool,
    #[clap(short, long, default_value_t = false)]
    pub info: bool,
    #[clap(short, long, default_value_t = false)]
    pub error: bool,
    #[clap(short, long, default_value_t = false)]
    pub trace: bool,
}
