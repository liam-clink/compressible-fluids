# Must only have functions, this will not be run in full
import matplotlib.pyplot as plt
import numpy as np

file = open("test_data/data.tsv")

data = []
for i in range(15):
    data.append(np.fromstring(file.readline(), sep="\t"))

plt.plot(data[0])
plt.plot(data[1])
plt.show()
