# -*- coding: utf-8 -*-

values = input().split(" ")

a = float(values[0])
b = float(values[1])
c = float(values[2])

if a < b + c and b < a + c and c < a + b:
    print(f"Perimetro = {(a + b + c):.1f}")
else:
    print(f"Area = {(a + b) * c / 2:.1f}")
