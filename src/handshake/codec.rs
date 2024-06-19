use bitcoin::{
    consensus::{deserialize_partial, serialize},
    p2p::message::RawNetworkMessage,
};
use bytes::{Buf, BytesMut};
use log::{info, warn, error};
use std::io;
use tokio_util::codec::{Decoder, Encoder};

/// A [codec](tokio_util::codec) implementation for the [Bitcoin protocol](https://en.bitcoin.it/wiki/Protocol_documentation).
pub struct BitcoinCodec {}

impl Decoder for BitcoinCodec {
    type Item = RawNetworkMessage;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match deserialize_partial::<RawNetworkMessage>(buf) {
            Ok((message, count)) => {
                buf.advance(count);
                info!("Successfully decoded message, advanced buffer by {}", count);
                Ok(Some(message))
            },
            Err(e) => {
                let io_error = match e {
                    bitcoin::consensus::encode::Error::Io(io_err) => {
                        warn!("I/O error during decoding: {}", io_err);
                        io_err
                    },
                    _ => {
                        error!("Encoding error during decoding");
                        io::Error::new(io::ErrorKind::Other, "Bitcoin encoding error")
                    },
                };
                if io_error.kind() == io::ErrorKind::UnexpectedEof {
                    info!("Data incomplete, waiting for more data");
                    Ok(None)
                } else {
                    error!("Error decoding data: {}", io_error);
                    Err(io_error)
                }
            }
        }
    }
}

impl Encoder<RawNetworkMessage> for BitcoinCodec {
    type Error = io::Error;

    fn encode(&mut self, item: RawNetworkMessage, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let data = serialize(&item);
        buf.extend_from_slice(&data);
        info!("Successfully encoded message");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handshake::message::build_version_message;
    use bitcoin::p2p::message::{NetworkMessage, RawNetworkMessage};
    use bitcoin::Network;
    use std::net::SocketAddr;

    #[test]
    fn test_codec_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let remote_address = "165.22.213.4:8333".parse::<SocketAddr>()?;
        let local_address = "0.0.0.0:0".parse::<SocketAddr>()?;
        let original = RawNetworkMessage::new(
            Network::Bitcoin.magic(),
            NetworkMessage::Version(build_version_message(&remote_address, &local_address)),
        );

        let mut bytes = BytesMut::new();
        BitcoinCodec {}
            .encode(original.clone(), &mut bytes)?;

        let deserialized = BitcoinCodec {}.decode(&mut bytes)?;
        assert_eq!(Some(original), deserialized);
        Ok(())
    }
}
