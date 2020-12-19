#!/usr/bin/env python

import re

def get_rules(file):
    fd = open(file)
    lines = fd.read().splitlines()
    fd.close()

    rules = {}
    words = []

    r_or = re.compile(r'^(\d+): (.*) \| (.*)$')
    r_char = re.compile(r'^(\d+): \"([a-z])\"$')
    r_rule = re.compile(r'^(\d+): (.*)$')

    for line in lines:
        res = r_or.match(line)
        if res:
            rules[res.groups()[0]] = (res.groups()[1].split(' '), res.groups()[2].split(' '))
            continue

        res = r_char.match(line)
        if res:
            rules[res.groups()[0]] = [res.groups()[1]]
            continue

        res = r_rule.match(line)
        if res:
            rules[res.groups()[0]] = res.groups()[1].split(' ')
            continue

        if line != '':
            words.append(line)

    return rules, words



def test(str, rule):
    if rules[rule].ischar():
        return str[0] == rules[rule]

def combine(rules, subrules):
    ret = []

    for e in subrules:
        ret.append(get_str(rules, e))

    while len(ret) > 1:
        nret = []
        for i in ret[0]:
            for j in ret[1]:
                nret.append(i + j)

        ret[0] = nret
        del(ret[1])

    return ret[0]


def get_str(rules, rule):
    if rule not in rules:
        return [rule]

    if isinstance(rules[rule], list):
        return combine(rules, rules[rule])

    if isinstance(rules[rule], tuple):
        return combine(rules, rules[rule][0]) + combine(rules, rules[rule][1])

    print(f"failed to match a rule type on rule {rule}")
    print(f"{rules[rule]}")

# 0: 8 11
# 8: 42 | 42 8
# 11: 42 31 | 42 11 31

def matches11(prefixes, suffixes, word):
    for prefix in prefixes:
        for suffix in suffixes:
            if len(word) < len(prefix) + len(suffix):
                continue

            if word.startswith(prefix) and word.endswith(suffix):
                if len(prefix) + len(suffix) == len(word):
                    return True

                if matches11(prefixes, suffixes, word[len(prefix):len(word)-len(suffix)]):
                    return True
    return False

def matches8(prefixes, suffixes, word):
    for prefix in prefixes:
        if len(word) < len(prefix):
            return False

        if not word.startswith(prefix):
            continue

        # word start with prefix. It's either a new 8's 42 or a 11's 42.
        return matches11(prefixes, suffixes, word[len(prefix):]) or matches8(prefixes, suffixes, word[len(prefix):])

    return False

rules, words = get_rules('input.txt')
possible = get_str(rules, "0")

i = 0
for word in words:
    if word in possible:
        i += 1
print(i)


rules, words = get_rules('input2.txt')

prefixes = get_str(rules, "42")
suffixes = get_str(rules, "31")

i = 0
for word in words:
    if matches8(prefixes, suffixes, word):
        i += 1
print(i)
