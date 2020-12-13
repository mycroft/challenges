#!/usr/bin/env python

fd = open('input.txt')
lines = fd.read().split('\n')
fd.close()

"""
direction % 360
east: 0
south: 90
west: 180
north: 270
"""

direction = 0

"""
x <- west --- east ->


"""
x = 0
y = 0

def get_direction():
    if direction % 360 == 0:
        return 'E'
    if direction % 360 == 90:
        return 'S'
    if direction % 360 == 180:
        return 'W'
    if direction % 360 == 270:
        return 'N'

for ins in lines:
    if ins == '':
        break

    inst = ins[0]
    code = int(ins[1:])

    if inst == 'L':
        direction += 360 - code
    if inst == 'R':
        direction += code

    if inst == 'F':
        inst = get_direction()

    if inst == 'N':
        y -= code
    if inst == 'S':
        y += code
    if inst == 'W':
        x -= code
    if inst == 'E':
        x += code

print(get_direction())
print(x, y, abs(x) + abs(y))

print("====")

direction = 0

x = 0
y = 0

wp_x = 10
wp_y = -1

for ins in lines:
    if ins == '':
        break

    inst = ins[0]
    code = int(ins[1:])

    print(inst, code)

    if inst == 'L':
        while code > 0:
            new_wp_y = wp_x * -1
            new_wp_x = wp_y

            wp_y = new_wp_y
            wp_x = new_wp_x

            code -= 90

    if inst == 'R':
        while code > 0:
            new_wp_y = wp_x
            new_wp_x = wp_y * -1

            wp_x = new_wp_x
            wp_y = new_wp_y

            code -= 90

    if inst == 'F':
        x += wp_x * code
        y += wp_y * code

    if inst == 'N':
        wp_y -= code
    if inst == 'S':
        wp_y += code
    if inst == 'W':
        wp_x -= code
    if inst == 'E':
        wp_x += code

    print(get_direction(), 'x', x, 'y', y, '~ wp_x', wp_x, 'wp_y', wp_y)

print(x, y, abs(x) + abs(y))
