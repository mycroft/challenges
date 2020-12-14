#!/usr/bin/env python

import re

fd = open('input.txt')
lines = fd.read().split('\n')
fd.close()

memory = {}

print('=== part 1 ===')


def setBit(int_type, offset):
    mask = 1 << offset
    return(int_type | mask)

def clearBit(int_type, offset):
    mask = ~(1 << offset)
    return(int_type & mask)

for line in lines:
    if line == '':
        break

    m = re.match(r'mask = (.*)', line)
    if m:
        mask = m.groups()[0]
        continue

    m = re.match(r'mem\[(.*)\] = (.*)$', line)
    if m:
        address = int(m.groups()[0])
        value = int(m.groups()[1])

    orig_value = value

    for i, bit in enumerate(mask):
        if bit == 'X':
            continue

        if bit == '1':
            value = setBit(value, 36 - i - 1)
        else:
            value = clearBit(value, 36 - i - 1)

    memory[address] = value

total = 0
for k in memory.keys():
    total += memory[k]

print('total is', total)


print('=== part 2 ===')

memory = {}

for line in lines:
    if line == '':
        break

    m = re.match(r'mask = (.*)', line)
    if m:
        mask = m.groups()[0]
        continue

    m = re.match(r'mem\[(.*)\] = (.*)$', line)
    if m:
        address = int(m.groups()[0])
        value = int(m.groups()[1])

    orig_value = value

    possibles_addresses = []

    count = 1
    for i, bit in enumerate(mask):
        if bit == 'X':
            count *= 2

    for c in range(count):
        possibles_addresses.append(address)

    vector = 1

    for i, bit in enumerate(mask):
        # 0
        # addresses: no change
        if bit == '0':
            continue

        # 1
        # addresses: set bit to 1
        if bit == '1':
            for idx, possible_address in enumerate(possibles_addresses):
                possibles_addresses[idx] = setBit(possible_address, 36 - i - 1)
            continue

        # X
        # addresses: both addresses
        odd = True

        for idx, possible_address in enumerate(possibles_addresses):
            if odd:
                possibles_addresses[idx] = setBit(possible_address, 36 - i - 1)
            else:
                possibles_addresses[idx] = clearBit(possible_address, 36 - i - 1)

            if idx % vector == 0:
                if odd:
                    odd = False
                else:
                    odd = True

        vector *= 2

    for idx, address in enumerate(possibles_addresses):
        memory[address] = value

total = 0
for k in memory.keys():
    total += memory[k]

print('total is', total)
