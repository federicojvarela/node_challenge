mod handshake;
mod model;
mod tools;

use anyhow::Context;
use clap::Parser;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    handshake::cli::init_tracing();
    let args = model::args::Args::parse();

    let remote_address = args
        .remote_address
        .parse::<SocketAddr>()
        .context("Invalid remote address")?;
    let local_address = args
        .local_address
        .parse::<SocketAddr>()
        .context("Invalid local address")?;
    let mut stream: Framed<TcpStream, handshake::codec::BitcoinCodec> =
        handshake::network::connect(&remote_address).await?;

    Ok(handshake::message::perform_handshake(&mut stream, &remote_address, local_address).await?)
}
