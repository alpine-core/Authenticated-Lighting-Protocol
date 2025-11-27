use alnp::{
    control::ControlClient,
    handshake::transport::{JsonUdpTransport, ReliableControlChannel, TimeoutTransport},
    messages::{ControlPayload, SetMode},
    session::{example_controller_session, LoopbackTransport},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let identity = alnp::messages::DeviceIdentity {
        cid: uuid::Uuid::new_v4(),
        manufacturer: "Demo".into(),
        model: "RustClient".into(),
        firmware_rev: "1.0.2".into(),
    };

    // Replace LoopbackTransport with JsonUdpTransport::bind for real network.
    // Refer to docs/implementation_audit.md for the UDP handshake/control/streaming architecture when wiring this example into production.
    let mut transport = LoopbackTransport::new();
    let session = example_controller_session(identity.clone(), &mut transport).await?;

    let signing = ed25519_dalek::SigningKey::from_bytes(&[1u8; 32]);
    let control = ControlClient::new(
        identity,
        alnp::control::ControlCrypto::new(signing, None),
    );

    let udp = JsonUdpTransport::bind("0.0.0.0:9001".parse()?, "127.0.0.1:9000".parse()?, 1200).await?;
    let mut reliable = ReliableControlChannel::new(TimeoutTransport::new(udp, std::time::Duration::from_millis(500)));

    let payload = ControlPayload::SetMode(SetMode {
        mode: alnp::messages::OperatingMode::Normal,
    });
    let ack = control.send(&mut reliable, payload).await?;
    println!("ACK: ok={} detail={:?}", ack.ok, ack.detail);

    // Use session + streaming adapter to send universes once Ready/Streaming.
    let sacn = alnp::stream::CSacnAdapter { source: 0, receiver: 0 };
    let stream = alnp::stream::AlnpStream::new(session, sacn);
    let _ = stream.send(1, &[0, 1, 2, 3]);
    Ok(())
}
