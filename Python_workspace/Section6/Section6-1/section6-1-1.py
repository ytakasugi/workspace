data = [
        11,22,33,44,55,
        66,77,
        88,99,100
]

print(data)

evenlist = list(range(0,10,2))
oddlist = list(range(1,10,2))

print(evenlist)
print(oddlist)

list_a = [["apple","peach","orange"],["cabbage","carrot","potato"]]
list_b = [[["p","y"],["t","y"]],[["o","n"],["3","note"]]]

print(list_a)
print(list_b)
print(list_b[0])

list_c = []

list_c.append(10)
list_c.append(20)
list_c.append(30)

print(list_c)

data.insert(11, 110)
print(data)

dessert = data.pop()
print(dessert)

fruits = ["apple", "orange", "banana", "peach"]
dessert = fruits.pop()
print(dessert)
print(fruits)

