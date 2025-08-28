# -*- coding: utf-8 -*-

values = input().split(" ")

a = int(values[0])
b = int(values[1])
c = int(values[2])

if a > b:
    a, b = b, a

if b > c:
    b, c = c, b

if a > b:
    a, b = b, a

print(f"{a}\n{b}\n{c}\n")
print(f"{values[0]}\n{values[1]}\n{values[2]}")
