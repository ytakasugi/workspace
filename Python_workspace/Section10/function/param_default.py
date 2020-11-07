def calc(size, num = 6):
    unit_price = {"S":120,"M":150,"L":180}
    price = unit_price[size] * num
    return (size,num,price)

a = calc("S",2)
print(f"{a[0]} Size,{a[1]} Pieces,{a[2]} yen")

b = calc("M")
print(f"{b[0]} Size,{b[1]} Pieces,{b[2]} yen")