from random import randint

while True:
    a = randint(1,13)
    b = randint(1,13)
    c = randint(1,13)

    if (a+b+c) == 21:
        break

print(a,b,c)