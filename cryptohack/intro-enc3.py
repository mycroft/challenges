#!/usr/bin/env python

from base64 import b64encode

input = '72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf'

print(b64encode(bytes.fromhex(input)).decode('ascii'))
