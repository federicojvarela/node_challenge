use crate::handshake::codec::BitcoinCodec;
use crate::handshake::error::ConectionError;

use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_util::codec::Framed;

use futures::TryFutureExt;

pub async fn connect(
    remote_address: &SocketAddr,
) -> Result<Framed<TcpStream, BitcoinCodec>, ConectionError> {
    let connection = TcpStream::connect(remote_address).map_err(ConectionError::ConnectionFailed);
    let stream = timeout(Duration::from_millis(500), connection)
        .map_err(ConectionError::ConnectionTimedOut)
        .await??;
    Ok(Framed::new(stream, BitcoinCodec {}))
}
