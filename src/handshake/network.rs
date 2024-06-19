use crate::handshake::codec::BitcoinCodec;
use crate::handshake::error::ConnectionError;

use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_util::codec::Framed;

use futures::TryFutureExt;

const TIMEOUT_DURATION: Duration = Duration::from_millis(500);

// Function to create a TcpStream connection to a given remote address.
// It returns a TcpStream wrapped in a Result, handling connection errors.
async fn create_tcp_stream(remote_address: &SocketAddr) -> Result<TcpStream, ConnectionError> {
    TcpStream::connect(remote_address)
        .map_err(ConnectionError::ConnectionFailed)
        .await
}

// Function to apply a timeout to a connection attempt.
// It takes a future representing the connection attempt and a duration for the timeout.
// Returns a Result with either the established TcpStream or a ConectionError if timed out or other errors occur.
async fn apply_timeout_to_connection(
    connection: impl std::future::Future<Output = Result<TcpStream, ConnectionError>>,
    duration: Duration,
) -> Result<TcpStream, ConnectionError> {
    match timeout(duration, connection).await {
        Ok(result) => result,
        Err(e) => Err(ConnectionError::ConnectionTimedOut(e)),
    }
}

// Public function to establish a connection to a remote address and wrap the connection in a BitcoinCodec.
// Returns a Framed<TcpStream, BitcoinCodec> which can be used to send and receive Bitcoin protocol messages.
pub async fn connect(
    remote_address: &SocketAddr,
) -> Result<Framed<TcpStream, BitcoinCodec>, ConnectionError> {
    let connection_future = create_tcp_stream(remote_address);
    let stream = apply_timeout_to_connection(connection_future, TIMEOUT_DURATION).await?;
    Ok(Framed::new(stream, BitcoinCodec {}))
}
