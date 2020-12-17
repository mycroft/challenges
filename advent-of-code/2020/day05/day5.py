#!/usr/bin/env python

fd = open('input.txt')
contents = fd.read().split('\n')
fd.close()

max_id = 0

all_ids = []

for line in contents:
    if line == '':
        break

    min_p = 0
    max_p = 127

    min_c = 0
    max_c = 7

    for c in line:
        s = (max_p+1) - min_p
        s2 = (max_c+1) - min_c

        if c == 'F' or c == 'L':
            max_p = int(max_p - s / 2)
            if c == 'L':
                max_c = int(max_c - s2 / 2)

        if c == 'B' or c == 'R':
            min_p = int(min_p + s / 2)
            if c == 'R':
                min_c = int(min_c + s2 / 2)

        # print(line, c, s, min_p, max_p, max(min_p, max_p), min_c, max_c)

    print(max(min_p, max_p), max(min_c, max_c), max(min_p, max_p) * 8 + max(min_c, max_c))

    all_ids.append(max(min_p, max_p) * 8 + max(min_c, max_c))

    if max(min_p, max_p) * 8 + max(min_c, max_c) > max_id:
        max_id = max(min_p, max_p) * 8 + max(min_c, max_c)


all_ids.sort()

print(max_id, all_ids)

i = 68
while i < 970:
    if i in all_ids:
        i = i + 1
        continue

    if i -1 in all_ids and i + 1 in all_ids:
        print(i)

    i = i + 1


