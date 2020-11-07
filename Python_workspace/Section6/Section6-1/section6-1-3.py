pos = int(input("Positon to take out:"))
colors = ["blue", "red", "green", "yellow"]
length = len(colors)
try:
    item = colors[pos]
    print(item)
except IndexError:
    print("indexerror")
except Exception as error:
    print("error")
    
    