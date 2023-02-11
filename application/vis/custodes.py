from tkinter import *
import sys
import json
from random import randint
root = Tk()


lab = Label(root)
lab.pack()

canvas = Canvas()
num_rows = 3
num_cols = 3
width = 50
height = 50
tolerance = 0.01

root.geometry(str(num_rows*width)+"x"+str(num_cols*height)+"+"+str(0)+"+"+str(0))

def prop_to_hex(prop):
    x = int(prop*255)
    return '#{:02x}{:02x}{:02x}'.format(x, x, x)


# class cell:
#     def __init__(self):
#         self.width = width
#         self.height = height
#         self.color = (0, 0, 0)
#


for row in range(num_rows):
    for col in range(num_cols):
        canvas.create_rectangle(row*width, col*height, row*width+width, col*height+height, fill=prop_to_hex(0))
canvas.pack(fill=BOTH, expand=1)\

def getMax(mat):
    max = 0
    for i in range(len(mat)):
        for j in range(len(mat[0])):
            if mat[i][j] > max:
                max = mat[i][j]
    return max

last_mat = None
def update():
    global last_mat
    mtext = sys.stdin.readline()

    print('\n\n\n', mtext, '\n\n\n')
    mat = json.loads(mtext)
    print('\n\n\n', mat, '\n\n\n')
    max = getMax(mat)

    for row in range(len(mat)):
        for col in range(len(mat[0])):
            val = mat[row][col]/max
            if last_mat != None:
                if abs(val - last_mat[row][col]) > tolerance:
                    canvas.create_rectangle(row * width, col * height, row * width + width, col * height + height,
                                    fill=prop_to_hex(val))
            mat[row][col] = val
    canvas.pack(fill=BOTH, expand=1)

    root.after(500, update)
    last_mat = mat



# # run first time
update()

root.mainloop()
