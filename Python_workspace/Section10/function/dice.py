from random import randint

def dice():
    num = randint(1,6)
    return num

def dicegame():
    dice1 = dice()
    dice2 = dice()
    sum = dice1 + dice2
    if sum%2 == 0:
        print(f"Sum of {dice1} and {dice2} is {sum} ,even number")
    else:
        print(f"Sum of {dice1} and {dice2} is {sum} ,odd number")

for i in range(5):
    dicegame()
print("end of game")