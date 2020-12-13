#!/usr/bin/env python

import sys

fd = open('input.txt')
lines = fd.read().split('\n')
fd.close()

adapters = [0]

for line in lines:
    if line == '':
        break

    adapter = int(line)
    adapters.append(adapter)

diff1 = 0
diff2 = 0
diff3 = 0

highest = 0

for adapter in adapters:
    if adapter + 1 in adapters:
        diff1 += 1
        continue

    if adapter + 2 in adapters:
        diff2 += 1
        continue

    if adapter + 3 in adapters:
        diff3 += 1
        continue

    if adapter + 3 > highest:
        highest = adapter + 3

print('1:', diff1, '2:', diff2, '3:', diff3, 'highest:', highest)


all_sets = []
adapters.sort()

mul = 1
current_group_set = []

def get_possible_pathes(sets):
    all_sets = [[]]

    for current_set in sets:
        new_all_sets = []
        for num in current_set:

            for all_set in all_sets:
                new_all_set = all_set.copy()
                if num not in new_all_set:
                    new_all_set.append(num)

                new_all_set.sort()

                new_all_sets.append(new_all_set)

        all_sets = new_all_sets

    final_set = []

    for all_set in all_sets:
        if all_set not in final_set:
            final_set.append(all_set)

    return (len(final_set))

for adapter in adapters:
    possibilities = []

    if adapter + 1 in adapters:
        possibilities.append(adapter + 1)
    if adapter + 2 in adapters:
        possibilities.append(adapter + 2)
    if adapter + 3 in adapters:
        possibilities.append(adapter + 3)

    #if len(possibilities) == 0:
    #    break

    print("possibilities for", adapter, ":", possibilities)
    if len(possibilities) == 1 and len(current_group_set) == 0:
        continue

    if len(current_group_set) == 0 and len(possibilities) > 1:
        current_group_set = [possibilities]
        continue

    found = False
    for possibility in possibilities:
        for current_subset in current_group_set:
            if possibility in current_subset:
                found = True
                break
        if found:
            break

    if not found:
        possibles_path = get_possible_pathes(current_group_set)
        print(len(current_group_set), 'num possible pathes', possibles_path, mul, mul * possibles_path)
        mul *= possibles_path
        current_group_set = []
    else:
        current_group_set.append(possibilities)

print(mul)
