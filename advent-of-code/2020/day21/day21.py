#!/usr/bin/env python

# Each allergen is found in exactly one ingredient.
# Each ingredient contains zero or one allergen.
# Allergens aren't always marked

import re

fd = open('input.txt')
lines = fd.read().splitlines()
fd.close();

ingredients = {}

alergens = {}

p = re.compile(r'^(.*) \(contains (.*)\)$')

for line in lines:
    if line == '':
        break

    m = p.match(line)

    ing = m.groups()[0].split(' ')
    al = m.groups()[1].split(' ')

    for ingredient in ing:
        if ingredient not in ingredients:
            ingredients[ingredient] = 0
        ingredients[ingredient] += 1

    for alergen in al:
        alergen = alergen.replace(',', '')
        if alergen not in alergens:
            alergens[alergen] = set(ing)
        else:
            alergens[alergen] = alergens[alergen].intersection(set(ing))

poison = {}
found = False

while not found:
    found = True

    for al in alergens:
        if len(alergens[al]) == 1:
            poison[list(alergens[al])[0]] = al
        else:
            found = False
            for ing in list(alergens[al]):
                if ing in poison:
                    alergens[al].remove(ing)

total = 0

for ing in ingredients:
    if ing not in poison:
        total += ingredients[ing]

print(total)

ing = []
for k in sorted(alergens.keys()):
    ing.append(list(alergens[k])[0])

print(','.join(ing))