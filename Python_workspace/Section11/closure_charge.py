#closure definition
def charge(price):
    #function fact
    def calc(num):
        return price * num
    return calc

#make two closures
child = charge(400)
adult = charge(1000)

#calculate the fee
price1 = child(3)
price2 = adult(2)
print(price1)
print(price2)