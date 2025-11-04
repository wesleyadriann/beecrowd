# -*- coding: utf-8 -*-

size = int(input())

values = input().split(" ")

small = int(values[0])
small_index = 0

for i in range(1, size):
    value = int(values[i])
    if value < small:
        small = value
        small_index = i

print(small_index + 1)
