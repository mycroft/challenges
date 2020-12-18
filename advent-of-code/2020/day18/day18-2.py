#!/usr/bin/env python

import re

fd = open('input.txt')
lines = fd.read().splitlines()
fd.close()

def getsub(expr):
    start = -1
    end = -1
    for i, c in enumerate(expr):
        if c == '(':
            start = i
            continue

        if c == ')':
            end = i
            return (start, end)

    return (start, end)

def eval1(expr):
    while len(expr) > 1:
        if expr[1] == '*':
            expr[2] = expr[0] * expr[2]
        else:
            expr[2] = expr[0] + expr[2]

        expr = expr[2:]

    return expr[0]

def eval2(expr):
    while len(expr) > 1:
        if '+' in expr:
            idx = expr.index('+')
            expr[idx + 1] = expr[idx - 1] + expr[idx + 1]
            expr = expr[:idx - 1] + expr[idx + 1:]
            continue

        expr[2] = expr[0] * expr[2]
        expr = expr[2:]

    return expr[0]

def reduce(expr, method):
    while True:
        sub = getsub(expr)
        if sub[0] >= 0:
            val = method(expr[sub[0]+1:sub[1]])

            expr = expr[:sub[0]] + [val] + expr[1+sub[1]:]
        else:
            break

    return method(expr)

total1 = 0
total2 = 0

for line in lines:
    expr1 = list(line.replace(' ', ''))
    for i, c in enumerate(expr1):
        if c.isnumeric():
            expr1[i] = int(c)
    expr2 = expr1.copy()

    total1 += reduce(expr1, eval1)
    total2 += reduce(expr2, eval2)

print(total1)
print(total2)
