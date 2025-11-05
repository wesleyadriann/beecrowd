# -*- coding: utf-8 -*-

line = 1
while True:
    try:
        value = input().split(" ")

        x = float(value[0])
        y = float(value[1])

        result = (y / x - 1) * 100

        print(f"Projeto {line}:")
        print(f"Percentual dos juros da aplicacao: {result:.2f} %\n")
        line += 1
    except EOFError:
        break
