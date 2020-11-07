def triangle_area(base,height):
    area = base * height / 2
    return area

b = 15 #底辺
h = 13 #高さ
v = triangle_area(b,h)
print(f"The area of bottom{b},height{h} is triangle {v:.1f}")