# Must only have functions, this will not be run in full
import matplotlib.pyplot as plt
import numpy as np


data = np.loadtxt("test_data/data.tsv")

plt.plot(data)
plt.show()
