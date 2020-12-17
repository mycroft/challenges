#!/usr/bin/env python

import re

fd = open('input.txt', 'r')
content = fd.read().split('\n\n')
fd.close()

required = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'] # cid

valid = 0
invalid = 0

for passport in content:
    isvalid = True

    h = {}

    values = re.findall(r'([^\s]*):([^\s]*)', passport)
    for value in values:
        h[value[0]] = value[1]

    for field in required:
        if field not in h.keys():
            isvalid = False
            break

        if field == 'byr' and (int(h[field]) < 1920 or int(h[field]) > 2002):
            print('invalid byr', h[field])
            isvalid = False
            break

        if field == 'iyr' and (int(h[field]) < 2010 or int(h[field]) > 2020):
            print('invalid iyr', h[field])
            isvalid = False
            break

        if field == 'eyr' and (int(h[field]) < 2020 or int(h[field]) > 2030):
            print('invalid eyr', h[field])
            isvalid = False
            break

        if field == 'hgt':
            suffix = h[field][-2:]
            num = h[field][:-2]

            if suffix not in ['cm', 'in']:
                print('invalid hgt', h[field])
                isvalid = False
                break

            if suffix == 'cm' and (int(num) < 150 or int(num) > 193):
                print('invalid hgt', h[field])
                isvalid = False
                break

            if suffix == 'in' and (int(num) < 59 or int(num) > 76):
                print('invalid hgt', h[field])
                isvalid = False
                break

        if field == 'hcl':
            if not re.match(r'^#[a-f0-9]{6}$', h[field]):
                print('invalid hcl', h[field])
                isvalid = False
                break

        if field == 'ecl':
            valid_values = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
            if h[field] not in valid_values:
                isvalid = False
                break

        if field == 'pid':
            if not re.match('^[0-9]{9}$', h[field]):
                print('invalid pid', h[field])
                isvalid = False
                break



    if isvalid:
        valid += 1
    else:
        invalid +=1

print('valid:', valid, 'invalid:', invalid)