from __future__ import annotations

import socket
import os
from typing import Dict, List, Optional, Tuple

from .. import build_discovery_request, encode_frame, encode_control, _to_cbor
from .. import CapabilitySet, ControlEnvelope, FrameEnvelope
from .profile import StreamProfile


class AlpineClient:
    def __init__(
        self,
        remote: Tuple[str, int],
        local: Tuple[str, int] = ("0.0.0.0", 0),
        timeout: float = 3.0,
    ):
        self._socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self._socket.settimeout(timeout)
        self._socket.bind(local)
        self._remote = remote
        self._session_id: Optional[str] = None
        self._config_id: Optional[str] = None

    def discover(self, requested: List[str], nonce: Optional[bytes] = None) -> Dict:
        nonce = nonce or os.urandom(32)
        request = build_discovery_request(requested, nonce)
        payload = _to_cbor(request.to_map())
        self._socket.sendto(payload, self._remote)
        data, _ = self._socket.recvfrom(2048)
        import cbor2

        return cbor2.loads(data)

    def send_frame(
        self,
        frame: FrameEnvelope,
        destination: Optional[Tuple[str, int]] = None,
    ) -> None:
        payload = _to_cbor(frame.to_map())
        self._socket.sendto(payload, destination or self._remote)

    def control(self, envelope: ControlEnvelope) -> None:
        payload = _to_cbor(envelope.to_map())
        self._socket.sendto(payload, self._remote)

    def close(self) -> None:
        self._socket.close()

    def start_stream(self, profile: Optional[StreamProfile] = None) -> str:
        """Bind the stream profile and return the runtime config_id.

        Keeps the selected profile immutable for the active session.
        """
        profile = profile or StreamProfile.auto()
        compiled = profile.compile()
        self._config_id = compiled.config_id
        return compiled.config_id
