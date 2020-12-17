#!/usr/bin/env python

import re

fd = open('input.txt')
lines = fd.read().split('\n')
fd.close()

rules = {}

r = re.compile(r'(.*) bags contain (.*).')
r2 = re.compile(r'(\d) (.*) bags?')

for line in lines:
    if line == "":
        break

    m = r.match(line)

    holder = m.groups()[0]
    holdees = m.groups()[1].split(', ')

    rules[holder] = {}

    if m.groups()[1] == 'no other bags':
        continue

    for unparsed_holdee in holdees:
        m2 = r2.match(unparsed_holdee)
        num = m2.groups()[0]
        color = m2.groups()[1]

        print(holder, '=', num, '/', color)

        rules[holder][color] = num


""" combien de sac dans un sac comme ça """
def getto(color):
    if rules[color] == {}:
        return 0

    num = 0

    print(rules[color])

    for child in rules[color]:
        print(color, child, rules[color][child])
        num += int(rules[color][child]) * (1 + getto(child))

    return num

print(rules)

print(getto('shiny gold'))


