# -*- coding: utf-8 -*-

while True:
    try:
        value = int(input())

        print(f"{value - 1}")
    except EOFError:
        break
