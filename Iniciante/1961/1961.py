# -*- coding: utf-8 -*-

p = int(input().split(" ")[0])

values = input().strip().split(" ")

win = True
prev = int(values[0])

for value in values[1:]:
    value = int(value)
    min = prev - p
    max = prev + p
    if value < min or value > max:
        win = False
        break
    prev = value

if win:
    print("YOU WIN")
else:
    print("GAME OVER")
