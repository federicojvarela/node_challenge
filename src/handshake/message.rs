use crate::handshake::codec::BitcoinCodec;
use crate::handshake::error::ConnectionError;
use crate::tools::config;

use bitcoin::p2p::message::{NetworkMessage, RawNetworkMessage};

use bitcoin::p2p::message_network::VersionMessage;
use bitcoin::p2p::{Address, ServiceFlags};
use bitcoin::Network;

use futures::{SinkExt, StreamExt};
use rand::Rng;
use std::net::SocketAddr;

use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tracing::{info, error, debug};

pub async fn perform_handshake(
    stream: &mut Framed<TcpStream, BitcoinCodec>,
    peer_address: &SocketAddr,
    local_address: SocketAddr,
) -> Result<(), ConnectionError> {
    let version_message = RawNetworkMessage::new(
        Network::Bitcoin.magic(),
        NetworkMessage::Version(build_version_message(peer_address, &local_address)),
    );

    info!("Sending version message to {}", peer_address);
    stream
        .send(version_message)
        .await
        .map_err(|e| {
            error!("Failed to send version message to {}: {}", peer_address, e);
            ConnectionError::SendingFailed(e)
        })?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => match message.payload() {
                NetworkMessage::Version(remote_version) => {
                    info!("Received version message from {}: {:?}", peer_address, remote_version);
                    let verack_message = RawNetworkMessage::new(
                        Network::Bitcoin.magic(),
                        NetworkMessage::Verack,
                    );
                    info!("Sending Verack to {}", peer_address);
                    stream
                        .send(verack_message)
                        .await
                        .map_err(|e| {
                            error!("Failed to send Verack to {}: {}", peer_address, e);
                            ConnectionError::SendingFailed(e)
                        })?;
                    return Ok(());
                }
                other_message => {
                    debug!("Received unsupported message from {}: {:?}", peer_address, other_message);
                }
            },
            Err(err) => {
                error!("Decoding error from {}: {}", peer_address, err);
                return Err(ConnectionError::ConnectionLost);
            }
        }
    }

    error!("Connection lost with {}", peer_address);
    Err(ConnectionError::ConnectionLost)
}

pub fn build_version_message(
    receiver_address: &SocketAddr,
    sender_address: &SocketAddr,
) -> VersionMessage {
    let start_height: i32 = config::get_env_var_as_int("START_HEIGHT");
    let user_agent: String = config::get_env_var_as_string("USER_AGENT");

    const SERVICES: ServiceFlags = ServiceFlags::NONE;

    let sender = Address::new(sender_address, SERVICES);
    let timestamp = chrono::Utc::now().timestamp();
    let receiver = Address::new(receiver_address, SERVICES);
    let nonce = rand::thread_rng().gen();

    VersionMessage::new(
        SERVICES,
        timestamp,
        receiver,
        sender,
        nonce,
        user_agent,
        start_height,
    )
}
