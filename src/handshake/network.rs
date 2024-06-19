use crate::handshake::codec::BitcoinCodec;
use crate::handshake::error::ConnectionError;

use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_util::codec::Framed;

use futures::TryFutureExt;

const TIMEOUT_DURATION: Duration = Duration::from_millis(500);

use log::{info, error};

async fn create_tcp_stream(remote_address: &SocketAddr) -> Result<TcpStream, ConnectionError> {
    info!("Attempting to connect to {}", remote_address);
    TcpStream::connect(remote_address)
        .map_err(|e| {
            error!("Failed to connect to {}: {}", remote_address, e);
            ConnectionError::ConnectionFailed(e)
        })
        .await
}

async fn apply_timeout_to_connection(
    connection: impl std::future::Future<Output = Result<TcpStream, ConnectionError>>,
    duration: Duration,
) -> Result<TcpStream, ConnectionError> {
    match timeout(duration, connection).await {
        Ok(result) => {
            info!("Connection established within timeout");
            result
        },
        Err(e) => {
            error!("Connection attempt timed out after {:?}", duration);
            Err(ConnectionError::ConnectionTimedOut(e))
        },
    }
}

pub async fn connect(
    remote_address: &SocketAddr,
) -> Result<Framed<TcpStream, BitcoinCodec>, ConnectionError> {
    info!("Starting connection process to {}", remote_address);
    let connection_future = create_tcp_stream(remote_address);
    let stream = apply_timeout_to_connection(connection_future, TIMEOUT_DURATION).await?;
    info!("Connection successfully established and wrapped with BitcoinCodec");
    Ok(Framed::new(stream, BitcoinCodec {}))
}
