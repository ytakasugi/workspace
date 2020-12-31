for i in range(4):
    for j in range(4):
        if i<j:
            print("." * j)
            continue
        print(f"i={i}, j={j}")