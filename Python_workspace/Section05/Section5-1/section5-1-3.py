from random import randint
point = randint(0,100)

if point >= 80 :
    result = "class A"
elif point >= 60 :
    result = "class B"
elif point >=  30 :
    result = "class C"
else:
    result = "Failure"

print(f"{point}:{result}")

