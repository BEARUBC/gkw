import busio
import digitalio
import board
import time
import sys
import time
import adafruit_mcp3xxx.mcp3008 as MCP
from adafruit_mcp3xxx.analog_in import AnalogIn

# create the spi bus
spi = busio.SPI(clock=board.SCK, MISO=board.MISO, MOSI=board.MOSI)

# create the cs (chip select)
cs = digitalio.DigitalInOut(board.D25) #Board.DX, X is equal to GPIO of whatever CS is on, so D25 = GPIO 25

# create the mcp object
mcp = MCP.MCP3008(spi, cs)

# create an analog input channel on pin 0

chan = AnalogIn(mcp, MCP.P0)
f = open("matlab2.txt", 'w')

while True:
    print('Raw ADC Value: ', chan.value)
    # print('ADC Voltage: ' + str(chan.voltage) + 'V')
    # print(str(chan.voltage))
    f.write(str(chan.value) + '\n')

