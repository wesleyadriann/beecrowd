# -*- coding: utf-8 -*-

values = input().split(" ")

bigger = 0

for value in values:
    value = int(value)
    if value > bigger:
        bigger = value

print(bigger)
