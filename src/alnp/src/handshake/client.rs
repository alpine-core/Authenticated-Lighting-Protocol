use async_trait::async_trait;
use uuid::Uuid;

use super::{
    HandshakeContext, HandshakeError, HandshakeMessage, HandshakeOutcome, HandshakeParticipant,
    HandshakeTransport,
};
use crate::crypto::{compute_mac, KeyExchange};
use crate::messages::{
    CapabilitySet, DeviceIdentity, MessageType, SessionAck, SessionEstablished, SessionInit,
    SessionReady,
};

/// Controller-side handshake driver implementing the ALPINE 1.0 flow.
pub struct ClientHandshake<A, K>
where
    A: super::ChallengeAuthenticator + Send + Sync,
    K: KeyExchange + Send + Sync,
{
    pub identity: DeviceIdentity,
    pub capabilities: CapabilitySet,
    pub authenticator: A,
    pub key_exchange: K,
    pub context: HandshakeContext,
}

#[async_trait]
impl<A, K> HandshakeParticipant for ClientHandshake<A, K>
where
    A: super::ChallengeAuthenticator + Send + Sync,
    K: KeyExchange + Send + Sync,
{
    async fn run<T: HandshakeTransport + Send>(
        &self,
        transport: &mut T,
    ) -> Result<HandshakeOutcome, HandshakeError> {
        let controller_nonce = super::new_nonce().to_vec();
        let session_id = Uuid::new_v4();

        // 1) Controller -> device: session_init
        let init = SessionInit {
            message_type: MessageType::SessionInit,
            controller_nonce: controller_nonce.clone(),
            controller_pubkey: self.key_exchange.public_key(),
            requested: self.capabilities.clone(),
            session_id,
        };
        transport.send(HandshakeMessage::SessionInit(init)).await?;

        // 2) Device -> controller: session_ack
        let ack = match transport.recv().await? {
            HandshakeMessage::SessionAck(ack) => ack,
            other => {
                return Err(HandshakeError::Protocol(format!(
                    "expected SessionAck, got {:?}",
                    other
                )))
            }
        };
        validate_ack(&ack, session_id, &controller_nonce, &self.context)?;

        // 3) Verify device signature over the controller nonce.
        let sig_valid = self
            .authenticator
            .verify_challenge(&controller_nonce, &ack.signature);
        if !sig_valid {
            return Err(HandshakeError::Authentication(
                "device signature validation failed".into(),
            ));
        }

        // 4) Derive shared keys (HKDF over concatenated nonces).
        let mut salt = controller_nonce.clone();
        salt.extend_from_slice(&ack.device_nonce);
        let keys = self
            .key_exchange
            .derive_keys(&ack.device_pubkey, &salt)
            .map_err(|e| HandshakeError::Authentication(format!("{}", e)))?;

        // 5) Controller -> device: session_ready (MAC proves key possession).
        let mac = compute_mac(&keys, 0, session_id.as_bytes(), ack.device_nonce.as_slice())
            .map_err(|e| HandshakeError::Authentication(e.to_string()))?;
        let ready = SessionReady {
            message_type: MessageType::SessionReady,
            session_id,
            mac,
        };
        transport
            .send(HandshakeMessage::SessionReady(ready))
            .await?;

        // 6) Device -> controller: session_complete
        let complete = match transport.recv().await? {
            HandshakeMessage::SessionComplete(c) => c,
            other => {
                return Err(HandshakeError::Protocol(format!(
                    "expected SessionComplete, got {:?}",
                    other
                )))
            }
        };
        if !complete.ok {
            return Err(HandshakeError::Authentication(
                "device rejected session_ready".into(),
            ));
        }

        let established = SessionEstablished {
            session_id,
            controller_nonce,
            device_nonce: ack.device_nonce,
            capabilities: ack.capabilities,
            device_identity: ack.device_identity,
        };

        Ok(HandshakeOutcome { established, keys })
    }
}

fn validate_ack(
    ack: &SessionAck,
    session_id: Uuid,
    controller_nonce: &[u8],
    context: &HandshakeContext,
) -> Result<(), HandshakeError> {
    if ack.session_id != session_id {
        return Err(HandshakeError::Protocol(
            "session_id mismatch between init and ack".into(),
        ));
    }

    if ack.device_nonce.len() != controller_nonce.len() {
        return Err(HandshakeError::Protocol(
            "device nonce length mismatch".into(),
        ));
    }

    if let Some(expected) = &context.expected_controller {
        if expected != &session_id.to_string() {
            return Err(HandshakeError::Authentication(
                "controller identity rejected".into(),
            ));
        }
    }

    Ok(())
}
