import datetime as dt
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import sys
# Create figure for plotting
fig = plt.figure()
ax = fig.add_subplot(1, 1, 1)
xs = []
ys = []
values = []

# Grabs data from EMG stdout 

# This function is called periodically from FuncAnimation
def animate(i, xs, ys):

    for line in sys.stdin:
        if line.replace("\n","") != "":
            values.append(float(line.replace("\n", "")))

    # Read voltage from EMG device

    # Add x and y to lists
    xs.append(dt.datetime.now().strftime('%H:%M:%S.%f'))
    
    if i == len(values):
        sys.exit("No more data")
        
    ys.append(values[i])
    i = i + 1
        

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
ani = animation.FuncAnimation(fig, animate, fargs=(xs, ys), interval=10)
plt.show()