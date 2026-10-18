#!/usr/bin/env python3
"""QEMUのモニタ(UNIXソケット)に screendump を送り、PPMをPNGに変換する(試験用)。"""
import socket
import struct
import sys
import time
import zlib

sock, ppm, png = sys.argv[1:4]
c = socket.socket(socket.AF_UNIX)
c.connect(sock)
c.recv(4096)
c.send(("screendump %s\n" % ppm).encode())
time.sleep(3)

d = open(ppm, "rb").read()
parts = d.split(b"\n", 3)
w, h = map(int, parts[1].split())
px = parts[3]
raw = b"".join(b"\x00" + px[y * w * 3:(y + 1) * w * 3] for y in range(h))


def chunk(t, b):
    return struct.pack(">I", len(b)) + t + b + struct.pack(">I", zlib.crc32(t + b) & 0xFFFFFFFF)


open(png, "wb").write(
    b"\x89PNG\r\n\x1a\n"
    + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0))
    + chunk(b"IDAT", zlib.compress(raw))
    + chunk(b"IEND", b"")
)
print("saved", png, w, h)
