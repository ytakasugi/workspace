numlist = [3, 4.2, 10, 1, 9]
sum = 0
for num in numlist:
    if not isinstance(num, (int,float)):
        print(num, "数値ではない値が含まれていました。")
        break
    sum += num
else:
    print("合計", sum)