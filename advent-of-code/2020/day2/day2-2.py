#!/usr/bin/env python

import re

fd = open('input.txt', 'r')
content = fd.read()
fd.close()

p = re.compile(r'^(\d+)-(\d+)\s(.):\s(.*)$')

valid = 0
invalid = 0

for line in content.split('\n'):
    if line == '':
        break

    g = p.match(line)

    c = 0

    if g[4][int(g[1]) -1] == g[3]:
        c += 1
    if g[4][int(g[2]) -1] == g[3]:
        c += 1

    if c == 1:
        valid += 1
    else:
        invalid += 1

print("valid:", valid)
print("invalid:", invalid)

