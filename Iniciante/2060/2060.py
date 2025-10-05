# -*- coding: utf-8 -*-

input()
values = input().split(" ")

result2 = 0
result3 = 0
result4 = 0
result5 = 0

for value in values:
    value = int(value)
    if value % 2 == 0:
        result2 += 1
    if value % 3 == 0:
        result3 += 1
    if value % 4 == 0:
        result4 += 1
    if value % 5 == 0:
        result5 += 1

print(f"{result2} Multiplo(s) de 2")
print(f"{result3} Multiplo(s) de 3")
print(f"{result4} Multiplo(s) de 4")
print(f"{result5} Multiplo(s) de 5")
