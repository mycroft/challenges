#!/usr/bin/env python

import re

fd = open('input.txt')
lines = fd.read().split('\n')
fd.close()

colors = {}

r = re.compile(r'(.*) bags contain (.*).')
r2 = re.compile(r'\d (.*) bags?')

for line in lines:
    if line == "":
        break

    m = r.match(line)

    if not m:
        print("failed for ", line)
        continue

    holder = m.groups()[0]

    if m.groups()[1] != 'no other bags':
        output = m.groups()[1].split(', ')
        for color in output:
            m2 = r2.match(color)
            holdee = m2.groups()[0]
            if holdee not in colors:
                colors[holdee] = []
            print(holdee, '/', holder, '/', colors[holdee])

            if holder not in colors[holdee]:
                colors[holdee].append(holder)

print(colors)

known_colors = []

def getholders(colors, z):
    print('getholders', z)
    if z not in colors:
        return []

    all_holders = []

    for holder in colors[z]:
        print('getholders', z, colors[z], holder)
        print('doing', colors[z])
        if holder not in all_holders:
            all_holders.append(holder)

        holders_temp = getholders(colors, holder)
        print(holders_temp)

        for sub_holder in holders_temp:
            print(sub_holder)
            if sub_holder not in all_holders:
                all_holders.append(sub_holder)

    print(z, 'can be contained in', all_holders)

    return all_holders


print(getholders(colors, 'shiny gold'))
print(len(getholders(colors, 'shiny gold')))
