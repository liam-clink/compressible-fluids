# Must only have functions, this will not be run in full
import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits import mplot3d


data = np.loadtxt("test_data/data.tsv")
X, Y = np.meshgrid(np.linspace(0, 1, data.shape[0]), np.linspace(0, 1, data.shape[0]))

ax = plt.axes(projection="3d")
ax.plot_surface(X, Y, data)
plt.show()
