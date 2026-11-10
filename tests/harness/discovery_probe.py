#!/usr/bin/env python3
"""
PocketMic Discovery Probe
Broadcasts a DISCOVER_REQ packet on port 37718 and listens for DISCOVER_ACK from the PC host.
"""

import socket
import json
import time
import argparse

DISCOVERY_PORT = 37718

def send_discovery(timeout_sec: float = 3.0, broadcast_ip: str = "255.255.255.255"):
    print(f"[PROBE] Emitting DISCOVER_REQ to {broadcast_ip}:{DISCOVERY_PORT}...")
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
    sock.settimeout(timeout_sec)

    req = {
        "cmd": "DISCOVER_REQ",
        "client_id": "test-probe-client",
        "device_name": "PocketMic-Probe-Device",
        "protocol_version": 1
    }

    req_data = json.dumps(req).encode('utf-8')
    t_start = time.time()
    sock.sendto(req_data, (broadcast_ip, DISCOVERY_PORT))

    try:
        data, addr = sock.recvfrom(2048)
        rtt_ms = (time.time() - t_start) * 1000.0
        resp = json.loads(data.decode('utf-8'))
        if resp.get("cmd") == "DISCOVER_ACK":
            print(f"[SUCCESS] Discovered Host: '{resp.get('server_name')}' at {addr[0]}:{resp.get('audio_port')} in {rtt_ms:.2f} ms")
            print(f"[DETAILS] Codecs: {resp.get('supported_codecs')} | SampleRate: {resp.get('sample_rate')} Hz")
            return resp
    except socket.timeout:
        print("[TIMEOUT] No DISCOVER_ACK received within timeout period.")
        return None
    finally:
        sock.close()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="PocketMic Discovery Probe")
    parser.add_argument("--broadcast", default="127.0.0.1", help="Broadcast/Target IP (default: 127.0.0.1 for local test)")
    parser.add_argument("--timeout", type=float, default=2.0, help="Timeout in seconds")
    args = parser.parse_args()

    send_discovery(timeout_sec=args.timeout, broadcast_ip=args.broadcast)
