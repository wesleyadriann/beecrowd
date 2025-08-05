# -*- coding: utf-8 -*-

while True:
    try:
        value = input()

        if value.strip() == "0":
            print("vai ter copa!")
        else:
            print("vai ter duas!")
    except EOFError:
        break
