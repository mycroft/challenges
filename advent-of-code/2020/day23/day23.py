#!/usr/bin/env python

import time

contents = "952316487" # my input
rounds = 100

# contents = "389125467" # test
# rounds = 10 # test

min_value = 1

part2 = False

if not part2:
    rounds = 100
    max_value = 9
else:
    rounds = 10000000
    max_value = 1000000


# Code is going here:

class Node:
    def __init__(self, value, prev, next):
        self.value = value
        self.prev = prev
        self.next = next

    def __str__(self):
        next_value = '???'
        if self.next:
            next_value = self.next.value
        prev_value = '???'
        if self.prev:
            prev_value = self.prev.value

        return f'Node: prev:{prev_value} <--> value:{self.value} <--> next:{next_value}'

root = None
prev = None

refs = {}

cups = []
for v in list(contents):
    cups.append(int(v))

if part2:
    for v in range(10, max_value+1):
        cups.append(v)

for v in cups:
    node = Node(int(v), prev, None)
    refs[int(v)] = node
    if prev:
        prev.next = node
    prev = node
    if not root:
        root = node

root.prev = node
node.next = root

def dump(count = 9):
    global root
    node = root

    for i in range(count):
        print(node)
        node = node.next

r = 1

while True:
    current_node = root

    pickup = current_node.next

    current_node.next = current_node.next.next.next.next
    current_node.next.prev = current_node

    guessed = current_node.value - 1

    while True:
        if guessed < 1:
            guessed = max_value
            continue

        if guessed in [pickup.value, pickup.next.value, pickup.next.next.value]:
            guessed -= 1
            continue

        destination = guessed
        break

    # print(f'destination: {destination}')

    destination_node = refs[destination]
    destination_node_next = destination_node.next

    # print(f'destination_node:      {destination_node}')
    # print(f'destination_node_next: {destination_node_next}')

    # print(f'pickup:     {pickup}')
    # print(f'pickup.n:   {pickup.next}')
    # print(f'pickup.n.n: {pickup.next.next}')

    destination_node.next = pickup
    pickup.prev = destination_node

    pickup.next.next.next = destination_node_next
    destination_node_next.prev = pickup.next.next

    root = root.next

    # dump()

    if r == rounds:
        break

    r += 1

root = refs[1]

if not part2:
    final = []
    for v in range(9):
        root = root.next
        final.append(str(root.value))

    print(''.join(final))
else:
    print(root.next.value * root.next.next.value)
