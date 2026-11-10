#!/usr/bin/env python3
"""
PocketMic FX Probe
Sends 8-byte binary FX control packets over UDP to switch effects or activate Bleep.
"""

import socket
import struct
import argparse

MAGIC = 0x504D
VERSION = 1
PACKET_TYPE_FX = 0x10

FX_MAP = {
    "bypass": 0,
    "normal": 0,
    "autotune": 1,
    "anime": 2,
    "chipmunk": 3,
    "demon": 4,
    "radio": 5,
    "robot": 6,
    "catedral": 8,
    "reverb": 8,
    "bleep": 9,
}

def send_fx_command(target_ip: str, target_port: int, effect_name: str, mode: int = 1, param1: int = 0):
    fx_id = FX_MAP.get(effect_name.lower())
    if fx_id is None:
        raise ValueError(f"Unknown effect: {effect_name}. Available: {list(FX_MAP.keys())}")

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    packet = struct.pack(">HBBB BH", MAGIC, VERSION, PACKET_TYPE_FX, fx_id, mode, param1)
    sock.sendto(packet, (target_ip, target_port))
    sock.close()
    print(f"[FX_PROBE] Sent FX_CMD: '{effect_name}' (ID: {fx_id}, Mode: {mode}, Param1: {param1}) to {target_ip}:{target_port}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="PocketMic FX Probe")
    parser.add_argument("effect", choices=list(FX_MAP.keys()), help="Effect name to activate")
    parser.add_argument("--target", default="127.0.0.1", help="Target IP")
    parser.add_argument("--port", type=int, default=37719, help="Target UDP port")
    parser.add_argument("--param", type=int, default=0, help="Optional numeric parameter")
    parser.add_argument("--mode", type=int, default=1, help="Mode (0=off, 1=on, 2=momentary)")
    args = parser.parse_args()

    send_fx_command(args.target, args.port, args.effect, mode=args.mode, param1=args.param)
