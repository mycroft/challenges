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

    count = 0
    for c in g[4]:
        if c == g[3]:
            count += 1

    if count >= int(g[1]) and count <= int(g[2]):
        valid += 1
        print("ok")
    else:
        invalid += 1
        print("not ok")

    print(g.groups())

print("valid: ", valid)
print("invalid: ", invalid)

