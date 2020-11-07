import numpy as np

vecx = np.array([[1,2,3],[4,5,6]])
vecy = np.array([[4,5,6],[7,8,9]])
vecz = np.array([[1,2,3],[7,8,9],[4,5,6]])

print(vecx + vecy)
print(vecx - vecy)

res = np.dot(vecx, vecz)
print(res)