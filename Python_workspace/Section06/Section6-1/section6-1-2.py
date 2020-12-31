pos = int(input("Positon to take out:"))
colors = ["blue", "red", "green", "yellow"]
length = len(colors)
if -length <= pos< length:
    item = colors[pos]
    print(item)
else:
    print("Error")
    