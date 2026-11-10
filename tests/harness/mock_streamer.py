#!/usr/bin/env python3
"""
PocketMic Mock Streamer
Generates 48 kHz synthetic audio (sine wave, voice tone) or captures from microphone,
encodes into Opus or PCM, formats with the 16-byte PocketMic header, and streams over UDP.
"""

import sys
import os
import time
import socket
import argparse
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

from protocol import AudioPacketHeader, PacketType, HeaderFlags

SAMPLE_RATE = 48000
FRAME_DURATION_MS = 10
SAMPLES_PER_FRAME = int(SAMPLE_RATE * (FRAME_DURATION_MS / 1000.0))  # 480 samples

def run_streamer(target_host: str, target_port: int, duration_sec: float, freq: float, packet_loss_rate: float, mute_window: tuple = None, simulate_clipping: bool = False):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    dest = (target_host, target_port)

    encoder = None
    if HAS_OPUS:
        try:
            encoder = opuslib.Encoder(SAMPLE_RATE, 1, opuslib.APPLICATION_RESTRICTED_LOWDELAY)
            print("[INFO] Opus Encoder initialized (@ 48kHz, mono, 10ms frames).")
        except Exception as e:
            print(f"[WARN] Could not initialize Opus Encoder ({e}), falling back to Float32 PCM.")
    else:
        print("[INFO] Opus library not loaded; streaming raw Float32 PCM.")

    print(f"[INFO] Streaming {duration_sec}s of {freq}Hz audio to {dest}...")
    print(f"[INFO] Frame size: {SAMPLES_PER_FRAME} samples ({FRAME_DURATION_MS} ms).")
    if packet_loss_rate > 0:
        print(f"[WARN] Simulating artificial packet loss of {packet_loss_rate * 100:.1f}%.")

    start_time = time.time()
    next_frame_time = start_time
    seq = 0
    phase = 0.0
    phase_increment = 2.0 * np.pi * freq / SAMPLE_RATE

    total_sent = 0
    total_dropped = 0

    try:
        while True:
            now = time.time()
            elapsed = now - start_time
            if duration_sec > 0 and elapsed >= duration_sec:
                break

            # Generate sine wave frame
            t = np.arange(SAMPLES_PER_FRAME)
            amplitude = 1.2 if simulate_clipping else 0.5
            samples = amplitude * np.sin(phase + t * phase_increment).astype(np.float32)
            phase = (phase + SAMPLES_PER_FRAME * phase_increment) % (2.0 * np.pi)

            # Check mute window
            is_muted = False
            if mute_window is not None:
                start_m, end_m = mute_window
                if start_m <= elapsed <= end_m:
                    is_muted = True

            # Check artificial packet loss
            if packet_loss_rate > 0 and np.random.rand() < packet_loss_rate:
                total_dropped += 1
                seq += 1
            else:
                flags = 0
                if is_muted:
                    flags |= HeaderFlags.MUTE
                    samples = np.zeros_like(samples)
                elif np.max(np.abs(samples)) > 0.95:
                    flags |= HeaderFlags.CLIPPING

                # Encode payload
                if encoder is not None:
                    # Clip to int16 range to avoid wrap-around overflow
                    clipped = np.clip(samples, -1.0, 1.0)
                    int16_samples = (clipped * 32767.0).astype(np.int16)
                    payload = encoder.encode(int16_samples.tobytes(), SAMPLES_PER_FRAME)
                else:
                    payload = samples.tobytes()

                ts_us = int((now - start_time) * 1_000_000) & 0xFFFFFFFF
                header = AudioPacketHeader(
                    packet_type=PacketType.AUDIO,
                    sequence_number=seq,
                    timestamp_us=ts_us,
                    flags=flags,
                    channels=1,
                    payload_len=len(payload),
                )

                datagram = header.encode() + payload
                sock.sendto(datagram, dest)
                total_sent += 1
                seq += 1

            next_frame_time += (FRAME_DURATION_MS / 1000.0)
            sleep_time = next_frame_time - time.time()
            if sleep_time > 0:
                time.sleep(sleep_time)

    except KeyboardInterrupt:
        print("\n[INFO] Streaming stopped by user.")

    print(f"[SUMMARY] Sent {total_sent} packets ({total_dropped} intentionally dropped). Done.")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="PocketMic Mock Streamer")
    parser.add_argument("--target", default="127.0.0.1", help="Target IP address")
    parser.add_argument("--port", type=int, default=37719, help="Target UDP port")
    parser.add_argument("--duration", type=float, default=5.0, help="Duration in seconds (0 = infinite)")
    parser.add_argument("--frequency", type=float, default=440.0, help="Sine wave frequency in Hz")
    parser.add_argument("--packet-loss", type=float, default=0.0, help="Artificial packet loss rate (0.0 to 1.0)")
    parser.add_argument("--mute-start", type=float, default=-1.0, help="Start time in seconds for simulated mute")
    parser.add_argument("--mute-end", type=float, default=-1.0, help="End time in seconds for simulated mute")
    parser.add_argument("--simulate-clipping", action="store_true", help="Simulate audio clipping with high amplitude")
    args = parser.parse_args()

    mute_window = None
    if args.mute_start >= 0 and args.mute_end > args.mute_start:
        mute_window = (args.mute_start, args.mute_end)

    run_streamer(args.target, args.port, args.duration, args.frequency, args.packet_loss, mute_window, args.simulate_clipping)
