#!/usr/bin/env python

import sys
from collections import defaultdict

fd = open('input.txt')
contents = fd.read()
fd.close()

initial_cube = set()

for y, line in enumerate(contents.split('\n')):
    for x, c in enumerate(line):
        if c == '#':
            initial_cube.add((x, y, 0))

def deltas():
    deltas = set(
        (x, y, z)
        for x in (-1, 0, 1)
        for y in (-1, 0, 1)
        for z in (-1, 0, 1)
    )

    deltas.remove((0, 0, 0))

    return deltas

def deltas_p2():
    deltas = set(
        (x, y, z, w)
        for x in (-1, 0, 1)
        for y in (-1, 0, 1)
        for z in (-1, 0, 1)
        for w in (-1, 0, 1)
    )

    deltas.remove((0, 0, 0, 0))

    return deltas

def run(current_cube):
    new_cube = set()
    neighbors = defaultdict(int)
    for cube in current_cube:
        for delta in deltas():
            neighbor = (
                delta[0] + cube[0],
                delta[1] + cube[1],
                delta[2] + cube[2],
            )

            neighbors[neighbor] += 1

    for neighbor in neighbors:
        if neighbors[neighbor] == 3:
            new_cube.add(neighbor)
        elif neighbors[neighbor] == 2 and neighbor in current_cube:
            new_cube.add(neighbor)

    return(new_cube)


def run_p2(current_cube):
    new_cube = set()
    neighbors = defaultdict(int)
    for cube in current_cube:
        for delta in deltas_p2():
            neighbor = (
                delta[0] + cube[0],
                delta[1] + cube[1],
                delta[2] + cube[2],
                delta[3] + cube[3],
            )

            neighbors[neighbor] += 1

    for neighbor in neighbors:
        if neighbors[neighbor] == 3:
            new_cube.add(neighbor)
        elif neighbors[neighbor] == 2 and neighbor in current_cube:
            new_cube.add(neighbor)

    return(new_cube)


current_cube = initial_cube

for _ in range(6):
    new_cube = run(current_cube)
    current_cube = new_cube

print("part 1:", len(current_cube))


current_cube = set(
    (c[0], c[1], 0, 0) for c in initial_cube
)

for _ in range(6):
    new_cube = run_p2(current_cube)
    current_cube = new_cube

print("part 2:", len(current_cube))
