sum = 50 + 37 + 10
limit = 100
if sum>=limit :
    result = "Pass"
else:
    result = "Failure"
    result += "/" + str(sum-limit)

print(sum)
print("-" * 20)
print(result)
        