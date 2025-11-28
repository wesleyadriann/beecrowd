# -*- coding: utf-8 -*-

n = int(input())

for i in range(n):
    values = input().split()

    o = "A porta fechou!" if values[2] == "0" else "A porta abriu!"

    print(f"{values[0].zfill(2)}:{values[1].zfill(2)} - {o}")
