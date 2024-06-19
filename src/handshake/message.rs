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

pub async fn perform_handshake(
    stream: &mut Framed<TcpStream, BitcoinCodec>,
    peer_address: &SocketAddr,
    local_address: SocketAddr,
) -> Result<(), ConnectionError> {
    let version_message = RawNetworkMessage::new(
        Network::Bitcoin.magic(),
        NetworkMessage::Version(build_version_message(peer_address, &local_address)),
    );

    stream
        .send(version_message)
        .await
        .map_err(ConnectionError::SendingFailed)?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => match message.payload() {
                NetworkMessage::Version(remote_version) => {
                    tracing::info!("Version message: {:?}", remote_version);
                    stream
                        .send(RawNetworkMessage::new(
                            Network::Bitcoin.magic(),
                            NetworkMessage::Verack,
                        ))
                        .await
                        .map_err(ConnectionError::SendingFailed)?;
                    return Ok(());
                }
                other_message => {
                    tracing::debug!("Unsupported message: {:?}", other_message);
                }
            },
            Err(err) => {
                tracing::error!("Decoding error: {}", err);
            }
        }
    }

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
