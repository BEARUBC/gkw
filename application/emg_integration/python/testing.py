import datetime as dt
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import time

fig = plt.figure()
ax = fig.add_subplot(1, 1, 1)
xs = []
ys = []
x = 1

def animate(i, xs, ys):
    # Read voltage from EMG device

    # Add x and y to lists
    xs.append(dt.datetime.now().strftime('%H:%M:%S.%f'))
    global x
    x = x / 2
    i = x
    print(x)
    ys.append(i)

    # Limit x and y lists to 20 items
    xs = xs[-20:]
    ys = ys[-20:]

    # Draw x and y lists
    ax.clear()
    ax.plot(xs, ys)

    # Format plot
    plt.xticks(rotation=45, ha='right')
    plt.subplots_adjust(bottom=0.30)
    plt.title('EMG Signals over Time')
    plt.ylabel('Voltage')

# Set up plot to call animate() function periodically
ani = animation.FuncAnimation(fig, animate, fargs=(xs, ys), interval=200)
plt.show()

