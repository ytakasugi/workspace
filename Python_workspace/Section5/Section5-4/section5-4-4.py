sum = 7600
while True:
    num = input("何人ですか？(qで終了)")
    if num == "q":
        print("終了しました。")
        break

    try:
        price = round(sum / int(num))
        
    except Exception as error:
        print("エラーになりました。")
        print(error)

    else:
        if price < 0:

            continue
        print("１人あたりの金額", price)