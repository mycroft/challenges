#!/usr/bin/env python

from pwn import xor

input = "73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d"
hidden = bytes.fromhex(input)

key = hidden[0] ^ ord('c')
print(f"Key is {key}.")
decoded = xor(hidden, hidden[0] ^ ord('c'))

print(decoded.decode("ascii"))
