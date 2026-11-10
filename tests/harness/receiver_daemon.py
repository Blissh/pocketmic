#!/usr/bin/env python3
"""
PocketMic Receiver Daemon (Python Test / Reference Host)
Receives UDP datagrams, parses 16-byte header, decodes Opus,
queues into JitterBuffer, and outputs to Windows WASAPI output device in real-time.
"""

import sys
import os
import time
import socket
import argparse
import threading
import numpy as np

# Ensure bundled DLLs from pyogg are in PATH before importing opuslib
try:
    import pyogg
    pyogg_dir = str(pyogg.__file__).rsplit(os.sep, 1)[0]
    os.environ['PATH'] = pyogg_dir + os.pathsep + os.environ.get('PATH', '')
except ImportError:
    pass

try:
    import opuslib
    HAS_OPUS = True
except Exception:
    HAS_OPUS = False

import sounddevice as sd
import json
import struct
from protocol import AudioPacketHeader, HEADER_SIZE, PacketType, HeaderFlags
from jitter_buffer import JitterBuffer
from fx_engine import FxEngine, VoiceEffectType

SAMPLE_RATE = 48000
FRAME_DURATION_MS = 10
SAMPLES_PER_FRAME = int(SAMPLE_RATE * (FRAME_DURATION_MS / 1000.0))  # 480 samples
DISCOVERY_PORT = 37718

def run_receiver(bind_host: str, bind_port: int, output_device=None, duration_sec: float = 0):
    print("=" * 60)
    print("🎙️  PocketMic Reference Receiver Daemon")
    print("=" * 60)

    # Initialize Discovery Socket
    disc_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    try:
        disc_sock.bind((bind_host, DISCOVERY_PORT))
        disc_sock.settimeout(0.5)
        print(f"[INFO] Discovery server listening on {bind_host}:{DISCOVERY_PORT}...")
    except Exception as e:
        print(f"[WARN] Discovery port could not be bound: {e}")
        disc_sock = None

    # Initialize Jitter Buffer
    jitter_buf = JitterBuffer(capacity_samples=SAMPLE_RATE // 10)  # 100 ms buffer
    print(f"[INFO] Jitter buffer allocated (Capacity: 100 ms / {jitter_buf.capacity} samples).")

    # Initialize Streamer FX Engine
    fx_engine = FxEngine(sample_rate=SAMPLE_RATE)
    print("[INFO] Streamer FX Engine initialized (Autotune, Anime, Memes, Bleep).")

    # Initialize Opus Decoder
    decoder = None
    if HAS_OPUS:
        try:
            decoder = opuslib.Decoder(SAMPLE_RATE, 1)
            print("[INFO] Opus Decoder initialized (@ 48kHz, mono).")
        except Exception as e:
            print(f"[WARN] Could not initialize Opus Decoder ({e}), falling back to Float32 PCM.")
    else:
        print("[INFO] Opus library not loaded; decoding Float32 PCM directly.")

    # Initialize UDP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind((bind_host, bind_port))
    sock.settimeout(0.5)
    print(f"[INFO] UDP socket listening on {bind_host}:{bind_port}...")

    # Audio callback for sounddevice (WASAPI)
    def audio_callback(outdata, frames, time_info, status):
        if status:
            pass # ignore underflows silently
        samples = jitter_buf.pop_or_silence(frames)
        outdata[:, 0] = samples

    # Start audio output stream
    stream = sd.OutputStream(
        samplerate=SAMPLE_RATE,
        channels=1,
        dtype='float32',
        device=output_device,
        blocksize=SAMPLES_PER_FRAME,
        callback=audio_callback
    )

    is_running = True
    stats = {
        'packets_received': 0,
        'packets_lost': 0,
        'last_seq': None,
        'start_time': time.time(),
        'peak_level': 0.0
    }

    def network_loop():
        nonlocal is_running
        while is_running:
            try:
                data, addr = sock.recvfrom(2048)
                if len(data) == 8 and data[0:2] == b'PM' and data[3] == 0x10:
                    # FX Control Command
                    _, _, _, fx_id, mode, param1 = struct.unpack(">HBBBBH", data)
                    if fx_id == VoiceEffectType.BLEEP:
                        fx_engine.set_bleep(mode != 0)
                        print(f"\n[FX] Bleep Censorship: {'ACTIVE' if mode != 0 else 'OFF'}")
                    else:
                        fx_engine.set_effect(fx_id)
                        print(f"\n[FX] Active Preset switched to ID {fx_id} (param={param1})")
                    continue

                if len(data) < HEADER_SIZE:
                    continue

                header = AudioPacketHeader.decode(data)
                payload = data[HEADER_SIZE:HEADER_SIZE + header.payload_len]

                # Sequence tracking
                if stats['last_seq'] is not None:
                    diff = header.sequence_number - stats['last_seq']
                    if diff > 1:
                        stats['packets_lost'] += (diff - 1)
                stats['last_seq'] = header.sequence_number
                stats['packets_received'] += 1

                # Decode audio
                if decoder is not None and len(payload) > 0:
                    pcm_bytes = decoder.decode(payload, SAMPLES_PER_FRAME)
                    pcm_int16 = np.frombuffer(pcm_bytes, dtype=np.int16)
                    samples = (pcm_int16.astype(np.float32) / 32767.0)
                else:
                    samples = np.frombuffer(payload, dtype=np.float32)

                if header.flags & HeaderFlags.MUTE:
                    samples = np.zeros_like(samples)

                # Process through Streamer FX Engine
                samples = fx_engine.process(samples)

                peak = np.max(np.abs(samples)) if len(samples) > 0 else 0.0
                stats['peak_level'] = max(stats['peak_level'] * 0.95, float(peak))

                # Push to JitterBuffer
                jitter_buf.push(samples)

            except socket.timeout:
                continue
            except Exception as e:
                if is_running:
                    print(f"[ERR] Network error: {e}")

    def discovery_loop():
        nonlocal is_running
        if disc_sock is None:
            return
        while is_running:
            try:
                data, addr = disc_sock.recvfrom(1024)
                req = json.loads(data.decode('utf-8'))
                if req.get("cmd") == "DISCOVER_REQ":
                    ack = {
                        "cmd": "DISCOVER_ACK",
                        "server_name": "PocketMic-Host-PC",
                        "audio_port": bind_port,
                        "protocol_version": 1,
                        "supported_codecs": ["opus"],
                        "sample_rate": SAMPLE_RATE
                    }
                    disc_sock.sendto(json.dumps(ack).encode('utf-8'), addr)
            except socket.timeout:
                continue
            except Exception:
                pass

    disc_thread = threading.Thread(target=discovery_loop, daemon=True)
    disc_thread.start()

    net_thread = threading.Thread(target=network_loop, daemon=True)
    net_thread.start()

    with stream:
        print(f"[INFO] Audio stream started on device: {stream.device} (WASAPI/PortAudio)")
        print("[INFO] PocketMic Daemon is live. Waiting for mobile audio stream...")
        try:
            start_loop = time.time()
            while is_running:
                time.sleep(0.5)
                elapsed = time.time() - start_loop
                if duration_sec > 0 and elapsed >= duration_sec:
                    break

                # Print live status
                total = stats['packets_received'] + stats['packets_lost']
                loss_rate = (stats['packets_lost'] / total * 100.0) if total > 0 else 0.0
                vu_bars = int(stats['peak_level'] * 20)
                vu_str = "█" * vu_bars + "░" * (20 - vu_bars)
                print(f"\r[STATUS] Pkts: {stats['packets_received']} | Lost: {stats['packets_lost']} ({loss_rate:.1f}%) | VU: [{vu_str}]", end="", flush=True)

        except KeyboardInterrupt:
            print("\n[INFO] Stopping daemon...")
        finally:
            is_running = False
            net_thread.join(timeout=1.0)
            disc_thread.join(timeout=1.0)
            sock.close()
            if disc_sock is not None:
                disc_sock.close()

    print("\n[INFO] Daemon stopped cleanly.")
    return stats

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="PocketMic Reference Receiver Daemon")
    parser.add_argument("--bind", default="0.0.0.0", help="Bind IP address")
    parser.add_argument("--port", type=int, default=37719, help="Bind UDP port")
    parser.add_argument("--duration", type=float, default=0, help="Duration in seconds (0 = infinite)")
    args = parser.parse_args()

    run_receiver(args.bind, args.port, duration_sec=args.duration)
