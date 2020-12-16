#!/usr/bin/env python

import re

fd = open('input.txt')
lines = fd.read().split('\n')
fd.close()

p = re.compile(r'(.*): (.*) or (.*)')
q = re.compile(r'^\d+,.*\d+$')

valid_values = []

nt = False
total = 0

valid_tickets = []

for line in lines:
    m = p.match(line)

    if m:
        v1 = m.groups()[1].split('-')
        v2 = m.groups()[2].split('-')

        valid_values.append(
            [
                (int(v1[0]), int(v1[1])),
                (int(v2[0]), int(v2[1]))
            ]
        )

    if not q.match(line):
        continue

    if q.match(line) and not nt:
        nt = True
        your_ticket = line.split(',')
        continue

    numbers = line.split(',')
    valid = True

    for number in numbers:
        found = False
        number = int(number)
        for valid_value in valid_values:
            if (number >= valid_value[0][0] and number <= valid_value[0][1]) or (number >= valid_value[1][0] and number <= valid_value[1][1]):
                found = True

        if not found:
            total += number
            valid = False

    if valid:
        valid_tickets.append(numbers)

print('total:', total)

def match(v, valid):
    return (v >= valid[0][0] and v <= valid[0][1]) or (v >= valid[1][0] and v <= valid[1][1])

# On veut les positions des 6 champs departure.

all_valids = []
for valid_value in valid_values:
    all_valids.append([])

for idx, valid in enumerate(valid_values):
    for idx_v in range(len(valid_values)):
        valid_rule = True
        for idx_t, ticket in enumerate(valid_tickets):
            if not match(int(ticket[idx_v]), valid):
                valid_rule = False

        if valid_rule:
            all_valids[idx_v].append(idx)

rules_index = {}
fixed_rules = []

while len(fixed_rules) != 20:
    for idx, all_valid in enumerate(all_valids):
        if len(all_valid) == 1 and all_valid[0] not in fixed_rules:

            fixed_rules.append(all_valid[0])
            rules_index[all_valid[0]] = idx

            for r in all_valids:
                if len(r) > 1 and all_valid[0] in r:
                    r.remove(all_valid[0])

total = 1

for k in rules_index:
    if k < 6:
        total *= int(your_ticket[int(rules_index[k])])

print("error rate", total)
