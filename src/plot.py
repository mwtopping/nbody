import numpy as np
import matplotlib.pyplot as plt


fig, ax = plt.subplots()

data = np.genfromtxt('output.txt')
colors=['blue', 'red', 'black']
for ii, row in enumerate(data):
    if ii%2 == 0:
        fig, ax = plt.subplots()

    ax.scatter(row[0], row[1], color=colors[ii%2])

    if ii%2 == 1:
        ax.set_xlim([-1.5, 1.5])
        ax.set_ylim([-1.5, 1.5])
        plt.show()
plt.show()
