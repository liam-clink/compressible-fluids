# Must only have functions, this will not be run in full
import matplotlib.pyplot as plt
matplotlib.use('Qt5Agg')

def basic_plot():
    plt.plot([0.,1.], [0.5, 1.])
    plt.show()