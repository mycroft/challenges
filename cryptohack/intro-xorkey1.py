#!/usr/bin/env python

# first, xor they encrypted with 'crypto{', find out the key starts with 'myXORke', add an 'y' and that's it!

from pwn import xor

input = "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104"
encrypted = bytes.fromhex(input)

pattern = "crypto{".encode("ascii")
key = xor(pattern, encrypted[0:len(pattern)])

# Added after first pass.
key = "myXORkey".encode("ascii")

print(f"Key is '{key.decode("ascii")}'.")
decoded = xor(encrypted, key)

# print(encrypted.decode("ascii"))
print(decoded.decode("ascii"))
