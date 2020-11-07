#料金を計算する関数
def calc(func,arg=1):
    price = func(arg)
    return price

#子供料金を計算する関数
def child(arg):
    return 400 * arg

#大人料金を計算する関数
def adult(arg):
    return 1200 * arg

#12歳,3人の料金を計算数する
age = 12
num = 3
if age < 16 :
    price = calc(child,num)
else:
    price = calc(adult,num)

print(f"{age}歳,{num}人で{price}円です。")