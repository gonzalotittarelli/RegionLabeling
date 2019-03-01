
import sys
import os
import time

def maxNeighbours(M, i, j):
    neighbour = []
    if(i > 0):
        neighbour.append(M[i-1][j])
    if(i < len(M)-1):
        neighbour.append(M[i+1][j])
    if(j > 0):
        neighbour.append(M[i][j-1])
    if(j < len(M[i])-1):
        neighbour.append(M[i][j+1])
    return max(neighbour)

def regionLabeling(M):
    start_time = time.time()
    m = len(M)
    n = len(M[0])
    R = [[0 for _ in range(n)] for _ in range(m)]
    for i in range(m):
        for j in range(n):
            if(M[i][j] == 1):
                R[i][j] = i*n+j
    while True:
        change = False
        for i in range(m):
            n = len(M[i])
            for j in range(n):
                oldlabel = R[i][j]
                if(M[i][j] == 1):
                    R[i][j] = maxNeighbours(R, i, j)
                if(R[i][j] != oldlabel):
                    change = True
        if(not change):
            break
    print("--- %s seconds ---" % (time.time() - start_time))
    return R

def printMatrix(M):
    s = [[str(e) for e in row] for row in M]
    lens = [max(map(len, col)) for col in zip(*s)]
    fmt = ''.join('{{:{}}}'.format(x) for x in lens)
    table = [fmt.format(*row) for row in s]
    print '\n'.join(table)
    #print('\n'.join([''.join(['{:}'.format(item) for item in row]) for row in M]))

def main():  
    filepath = sys.argv[1]

    M = []

    if not os.path.isfile(filepath):
       print("File path {} does not exist. Exiting...".format(filepath))
       sys.exit()

    with open(str(sys.argv[1]),"r") as f:
        line = f.readline()                                                                                                                                                                                                                                                                                                             
        while line:
            row = []
            for number in line.strip():
                row.append(int(number))
            line = f.readline()
            M.append(row)
    
    R = regionLabeling(M)
    m = len(R)
    n = len(R[0])

    for i in range(m):
        for j in range(n):
            if(R[i][j] > 0):
                R[i][j] = 5
    printMatrix(R)
if __name__ == '__main__':
   main()"""