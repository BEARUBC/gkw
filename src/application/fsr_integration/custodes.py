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
tolerance = 0
gamerMode = False

root.geometry(str(num_rows * width) + "x" + str(num_cols * height) + "+" + str(0) + "+" + str(0))


def prop_to_hex(prop):
    global gamerMode
    x = int(prop * 255)

    if gamerMode:
        r, g, b = 10, 10, 10
        if prop < 0.33:
            r = 1
        elif prop < 0.66:
            g = 1
        else:
            b = 1
    else:
        r, g, b = 1, 1, 1
    return '#{:02x}{:02x}{:02x}'.format(int(x / r), int(x / g), int(x / b))


# def get_max(mat):
#     max = 0
#     for i in range(len(mat)):
#         for j in range(len(mat[0])):
#             if mat[i][j] > max:
#                 max = mat[i][j]
#     return max


for row in range(num_rows):
    for col in range(num_cols):
        canvas.create_rectangle(row * width, col * height, row * width + width, col * height + height,
                                fill=prop_to_hex(0))
canvas.pack(fill=BOTH, expand=1)


last_mat = None
init = False

def update():
    global last_mat
    global init
    mtext = sys.stdin.readline()

    mat = json.loads(mtext)

    if not init:
        init = True
        last_mat = mat

    for row in range(len(mat)):
        for col in range(len(mat[0])):
            val = mat[row][col] / 255
            if abs(val - last_mat[row][col]) > tolerance:
                canvas.create_rectangle(row * width, col * height, row * width + width, col * height + height,
                                        fill=prop_to_hex(val))
                last_mat = mat

            mat[row][col] = val
    canvas.pack(fill=BOTH, expand=1)

    root.after(500, update)


# # run first time
update()

root.mainloop()
