use clap::Parser;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The address of the node to reach to.
    /// `dig seed.bitcoin.sipa.be +short` may provide a fresh list of nodes.
    #[arg(short, long, default_value = "165.22.213.4:8333")]
    remote_address: String,

    /// The address of this local node.
    /// This address doesn't matter much as it will be ignored by the bitcoind node
    #[arg(short, long, default_value = "0.0.0.0:0")]
    local_address: String,
}

pub fn init_tracing() {
    let env = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("RUST_LOG")
        .from_env_lossy();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env)
        .init();
}
