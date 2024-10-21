#!/usr/bin/env python

from pwn import xor

input = "label"
output = xor(input.encode("ascii"), 13).decode("ascii")

print(f"crypto{{{output}}}")
