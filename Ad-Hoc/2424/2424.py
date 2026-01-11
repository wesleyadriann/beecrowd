# -*- coding: utf-8 -*-

values = input().split(" ")

x = int(values[0])
y = int(values[1])

if x < 0 or y < 0 or x > 432 or y > 468:
    print("fora")
else:
    print("dentro")
