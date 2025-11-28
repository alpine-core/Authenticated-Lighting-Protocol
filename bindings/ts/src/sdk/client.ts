import dgram from "dgram";
import crypto from "crypto";
import cbor from "cbor";

import {
  buildDiscoveryRequest,
  buildFrameEnvelope,
  CapabilitySet,
  FrameEnvelope,
  MessageType,
} from "../index";
import { StreamProfile } from "../profile";

const DEFAULT_TIMEOUT = 3000;

export interface AlpineConnectionOptions {
  remotePort: number;
  remoteHost: string;
  localPort?: number;
}

export class AlpineClient {
  private socket: dgram.Socket;
  private readonly remoteHost: string;
  private readonly remotePort: number;
  private readonly timeout: number;

  constructor(options: AlpineConnectionOptions, timeout = DEFAULT_TIMEOUT) {
    this.remoteHost = options.remoteHost;
    this.remotePort = options.remotePort;
    this.timeout = timeout;
    this.socket = dgram.createSocket("udp4");
    this.socket.bind(options.localPort ?? 0);
  }

  async discover(requested: string[], nonce?: Buffer, capabilities?: CapabilitySet): Promise<Buffer> {
    const requestNonce = nonce ?? crypto.randomBytes(32);
    const request = buildDiscoveryRequest(requested, requestNonce);
    const payload = cbor.encode(request);
    await this.send(payload);
    return this.receive();
  }

  async sendFrame(frame: FrameEnvelope): Promise<void> {
    const payload = cbor.encode(frame);
    await this.send(payload);
  }

  async handshake(): Promise<Buffer> {
    const ready = Buffer.from(JSON.stringify({ type: MessageType.SessionInit }));
    await this.send(ready);
    return this.receive();
  }

  /**
   * Starts streaming with a declarative stream profile (default: Auto).
   *
   * Returns a deterministic `configId` that should be bound to the session and
   * never change while streaming is active.
   */
  async startStream(profile: StreamProfile = StreamProfile.auto()): Promise<string> {
    const configId = profile.configId();
    return configId;
  }

  close(): void {
    this.socket.close();
  }

  private async send(payload: Buffer): Promise<void> {
    return new Promise((resolve, reject) => {
      this.socket.send(payload, this.remotePort, this.remoteHost, (err) => {
        if (err) {
          reject(err);
        } else {
          resolve();
        }
      });
    });
  }

  private async receive(): Promise<Buffer> {
    return new Promise((resolve, reject) => {
      const onMessage = (msg: Buffer) => {
        cleanup();
        resolve(msg);
      };

      const onTimeout = () => {
        cleanup();
        reject(new Error("receive timeout"));
      };

      const cleanup = () => {
        this.socket.off("message", onMessage);
        clearTimeout(timer);
      };

      this.socket.once("message", onMessage);
      const timer = setTimeout(onTimeout, this.timeout);
    });
  }
}
