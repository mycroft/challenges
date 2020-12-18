#!/usr/bin/env python

import re

fd = open('input.txt')
lines = fd.read().splitlines()
fd.close()

p = re.compile(r'.*(\([0-9+*]*\)).*')
q = re.compile(r'^(\d+)([*+])(\d+)')

r = re.compile(r'.*?([0-9]+)\+([0-9]+)')
t = re.compile(r'^([0-9]+)\*([0-9]+)')

def compute(expr):
    while True:
        m = q.match(expr)

        if not m:
            break

        v1 = m.groups()[0]
        op = m.groups()[1]
        v2 = m.groups()[2]

        if op == '+':
            val = int(v1) + int(v2)
        else:
            val = int(v1) * int(v2)

        expr = expr.replace(m.group(), str(val), 1)

    return int(expr)

def compute2(expr):
    while True:
        m = r.match(expr)
        if m:
            v1 = m.groups()[0]
            v2 = m.groups()[1]

            val = int(v1) + int(v2)

            expr = expr.replace(v1 + '+' + v2, str(val), 1)
            continue

        m = t.match(expr)
        if m:
            v1 = m.groups()[0]
            v2 = m.groups()[1]

            val = int(v1) * int(v2)

            expr = expr.replace(v1 + '*' + v2, str(val), 1)
            continue

        break
    return int(expr)

total = 0

for line in lines:
    line = line.replace(' ', '')

    while True:
        m = p.match(line)
        if not m:
            break

        v = compute(m.groups()[0][1:-1])

        line = line.replace(m.groups()[0], str(v))

    total += compute(line)

print(total)


total = 0

for line in lines:
    line = line.replace(' ', '')

    while True:
        m = p.match(line)
        if not m:
            break

        v = compute2(m.groups()[0][1:-1])

        line = line.replace(m.groups()[0], str(v))

    total += compute2(line)

print(total)

