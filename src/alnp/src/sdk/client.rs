use std::collections::HashMap;
use std::fmt;
use std::net::SocketAddr;
use std::net::UdpSocket as StdUdpSocket;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::crypto::identity::NodeCredentials;
use crate::crypto::X25519KeyExchange;
use crate::control::{ControlClient, ControlCrypto};
use crate::handshake::keepalive;
use crate::handshake::transport::{CborUdpTransport, TimeoutTransport};
use crate::handshake::{ChallengeAuthenticator, HandshakeContext, HandshakeError};
use crate::messages::{CapabilitySet, ChannelFormat, ControlEnvelope, ControlOp, DeviceIdentity, MessageType};
use crate::session::{AlnpSession, AlnpRole};
use crate::stream::{AlnpStream, FrameTransport, StreamError};
use serde_json::Value;
use uuid::Uuid;

/// Errors emitted by the high-level SDK client.
#[derive(Debug)]
pub enum ClientError {
    Io(String),
    Handshake(HandshakeError),
    Stream(StreamError),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::Io(err) => write!(f, "io error: {}", err),
            ClientError::Handshake(err) => write!(f, "handshake error: {}", err),
            ClientError::Stream(err) => write!(f, "stream error: {}", err),
        }
    }
}

impl From<HandshakeError> for ClientError {
    fn from(err: HandshakeError) -> Self {
        ClientError::Handshake(err)
    }
}

impl From<StreamError> for ClientError {
    fn from(err: StreamError) -> Self {
        ClientError::Stream(err)
    }
}

impl From<std::io::Error> for ClientError {
    fn from(err: std::io::Error) -> Self {
        ClientError::Io(err.to_string())
    }
}

/// Thin UDP transport for the ALPINE streaming layer.
struct UdpFrameTransport {
    socket: StdUdpSocket,
    peer: SocketAddr,
}

impl UdpFrameTransport {
    fn new(local: SocketAddr, peer: SocketAddr) -> Result<Self, std::io::Error> {
        let socket = StdUdpSocket::bind(local)?;
        socket.connect(peer)?;
        Ok(Self { socket, peer })
    }
}

impl FrameTransport for UdpFrameTransport {
    fn send_frame(&self, bytes: &[u8]) -> Result<(), String> {
        self.socket
            .send(bytes)
            .map_err(|e| format!("udp stream send: {}", e))?;
        Ok(())
    }
}

/// High-level controller client that orchestrates the discovery, handshake, stream,
/// and keepalive flows.
pub struct AlpineClient {
    session: AlnpSession,
    transport: Arc<Mutex<TimeoutTransport<CborUdpTransport>>>,
    stream: AlnpStream<UdpFrameTransport>,
    control: ControlClient,
    keepalive_handle: Option<JoinHandle<()>>,
}

impl AlpineClient {
    /// Connects to a remote ALPINE device using the provided credentials.
    pub async fn connect(
        local_addr: SocketAddr,
        remote_addr: SocketAddr,
        identity: DeviceIdentity,
        capabilities: CapabilitySet,
        credentials: NodeCredentials,
    ) -> Result<Self, ClientError> {
        let key_exchange = X25519KeyExchange::new();
        let authenticator = crate::session::Ed25519Authenticator::new(credentials.clone());

        let mut transport =
            TimeoutTransport::new(CborUdpTransport::bind(local_addr, remote_addr, 2048).await?, Duration::from_secs(3));
        let session = AlnpSession::connect(
            identity,
            capabilities.clone(),
            authenticator,
            key_exchange,
            HandshakeContext::default(),
            &mut transport,
        )
        .await?;

        let transport = Arc::new(Mutex::new(transport));
        let keepalive_handle = tokio::spawn(keepalive::spawn_keepalive(
            transport.clone(),
            Duration::from_secs(5),
            session
                .established()
                .ok_or_else(|| ClientError::Io("session missing after handshake".into()))?
                .session_id,
        ));

        let stream_socket = UdpFrameTransport::new(local_addr, remote_addr)?;
        let stream = AlnpStream::new(session.clone(), stream_socket);

        let established = session
            .established()
            .ok_or_else(|| ClientError::Io("session missing after handshake".into()))?;
        let device_uuid = Uuid::parse_str(&established.device_identity.device_id)
            .unwrap_or_else(|_| Uuid::new_v4());
        let control_crypto = ControlCrypto::new(
            session
                .keys()
                .ok_or_else(|| ClientError::Io("session keys missing".into()))?,
        );
        let control = ControlClient::new(device_uuid, established.session_id, control_crypto);

        Ok(Self {
            session,
            transport,
            stream,
            control,
            keepalive_handle: Some(keepalive_handle),
        })
    }

    /// Sends a streaming frame via the high-level helper.
    pub fn send_frame(
        &self,
        channel_format: ChannelFormat,
        channels: Vec<u16>,
        priority: u8,
        groups: Option<HashMap<String, Vec<u16>>>,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<(), ClientError> {
        self.stream
            .send(channel_format, channels, priority, groups, metadata)
            .map_err(ClientError::from)
    }

    /// Gracefully closes the client, stopping keepalive tasks.
    pub async fn close(mut self) {
        self.session.close();
        if let Some(handle) = self.keepalive_handle.take() {
            handle.abort();
        }
    }

    /// Builds an authenticated control envelope ready for transport.
    pub fn control_envelope(
        &self,
        seq: u64,
        op: ControlOp,
        payload: Value,
    ) -> Result<ControlEnvelope, HandshakeError> {
        self.control.envelope(seq, op, payload)
    }
}
