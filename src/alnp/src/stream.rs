use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use thiserror::Error;

use crate::messages::{ChannelFormat, FrameEnvelope, MessageType};
use crate::session::{AlnpSession, JitterStrategy};

/// Minimal transport for sending serialized ALPINE frames (UDP/QUIC left to the caller).
pub trait FrameTransport: Send + Sync {
    fn send_frame(&self, bytes: &[u8]) -> Result<(), String>;
}

#[derive(Debug)]
pub struct AlnpStream<T: FrameTransport> {
    session: AlnpSession,
    transport: T,
    last_frame: parking_lot::Mutex<Option<FrameEnvelope>>,
}

#[derive(Debug, Error)]
pub enum StreamError {
    #[error("sender not authenticated")]
    NotAuthenticated,
    #[error("transport error: {0}")]
    Transport(String),
    #[error("streaming disabled")]
    StreamingDisabled,
    #[error("no session available")]
    MissingSession,
}

impl<T: FrameTransport> AlnpStream<T> {
    pub fn new(session: AlnpSession, transport: T) -> Self {
        Self {
            session,
            transport,
            last_frame: parking_lot::Mutex::new(None),
        }
    }

    pub fn set_jitter_strategy(&self, strat: JitterStrategy) {
        self.session.set_jitter_strategy(strat);
    }

    /// Sends a streaming frame built from raw channel data.
    pub fn send(
        &self,
        channel_format: ChannelFormat,
        channels: Vec<u16>,
        priority: u8,
        groups: Option<HashMap<String, Vec<u16>>>,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<(), StreamError> {
        let established = self
            .session
            .ensure_streaming_ready()
            .map_err(|_| StreamError::NotAuthenticated)?;
        if !self.session.streaming_enabled() {
            return Err(StreamError::StreamingDisabled);
        }

        let adjusted_channels = self.apply_jitter(&channels);

        let envelope = FrameEnvelope {
            message_type: MessageType::AlpineFrame,
            session_id: established.session_id,
            timestamp_us: Self::now_us(),
            priority,
            channel_format,
            channels: adjusted_channels,
            groups,
            metadata,
        };

        let bytes = serde_cbor::to_vec(&envelope)
            .map_err(|e| StreamError::Transport(format!("encode: {}", e)))?;
        self.transport
            .send_frame(&bytes)
            .map_err(StreamError::Transport)?;
        *self.last_frame.lock() = Some(envelope);
        Ok(())
    }

    fn apply_jitter(&self, channels: &[u16]) -> Vec<u16> {
        match self.session.jitter_strategy() {
            JitterStrategy::HoldLast => {
                if channels.is_empty() {
                    if let Some(last) = self.last_frame.lock().as_ref() {
                        return last.channels.clone();
                    }
                }
                channels.to_vec()
            }
            JitterStrategy::Drop => {
                if channels.is_empty() {
                    Vec::new()
                } else {
                    channels.to_vec()
                }
            }
            JitterStrategy::Lerp => {
                if let Some(last) = self.last_frame.lock().as_ref() {
                    let mut blended = Vec::with_capacity(channels.len());
                    for (idx, value) in channels.iter().enumerate() {
                        let prev = last.channels.get(idx).cloned().unwrap_or(0);
                        blended.push(((prev as u32 + *value as u32) / 2) as u16);
                    }
                    blended
                } else {
                    channels.to_vec()
                }
            }
        }
    }

    fn now_us() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64
    }
}
