use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio_util::codec::Framed;
use crate::codec::BitcoinCodec;
use crate::error::Error;

pub async fn connect(remote_address: &SocketAddr) -> Result<Framed<TcpStream, BitcoinCodec>, Error> {
    let connection = TcpStream::connect(remote_address).map_err(Error::ConnectionFailed);
    let stream = timeout(Duration::from_millis(500), connection)
        .map_err(Error::ConnectionTimedOut)
        .await??;
    Ok(Framed::new(stream, BitcoinCodec {}))
}