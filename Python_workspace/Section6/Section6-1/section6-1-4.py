colors = ["blue", "red", "yellow", "red", "green"]
print("Before Deletion",colors)
target = "red"
while target in colors:
    colors.remove(target)
print("After Deletion",colors)