from random import randint
tickets = 5
point = 0
fmt = "{:>3}"
while tickets>0:
    v = randint(1,100)
    print(fmt.format(v))
    point += v
    tickets -= 1

print("-" * 3)
print(fmt.format(point))

