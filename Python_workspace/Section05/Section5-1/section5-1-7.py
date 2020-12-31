from random import randint
a = randint(0,100)
b = randint(0,100)

bigger = a if a > b else b

text = f"{a}と{b}では、{bigger}が大きい"
print(text)