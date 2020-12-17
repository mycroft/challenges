#!/usr/bin/env python

fd = open('input2.txt')
contents = fd.read()
fd.close()

total = 0

for group in contents.split("\n\n"):
    group = group.replace('\n', '')
    group_letters = []

    for c in group:
        if c not in group_letters:
            group_letters.append(c)

    total += len(group_letters)

print(total)

total = 0
groups = list(map(lambda g: g.rstrip(), contents.split("\n\n")))

for group in groups:
    persons = group.split("\n")
    count = len(persons)

    letters = []

    for q in persons[0]:
        inall = True
        for person in persons[1:]:
            if q not in person:
                inall = False
                break

        if inall:
            letters.append(q)

    total += len(letters)

print(total)
