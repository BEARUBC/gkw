rows = 5
cols = 5


while True:
    for i in range(rows):
        mstr = '['
        for j in range(rows):
            mstr += '['
            for k in range(cols):
                mstr += str((k+i+j) % (rows))
                if k != cols-1:
                    mstr += ','
            mstr += ']'
            if j != rows - 1:
                mstr += ','
        mstr += ']'
        print(mstr)
        # time.sleep(1)
