#!/usr/bin/env python

from Crypto.Util.number import long_to_bytes

input = 11515195063862318899931685488813747395775516287289682636499965282714637259206269

print(long_to_bytes(input).decode('ascii'))
