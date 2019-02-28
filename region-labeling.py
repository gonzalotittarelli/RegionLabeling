
import sys
import os

def printMatrix(M):
    print('\n'.join([''.join(['{:}'.format(item) for item in row]) for row in M]))

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
    printMatrix(M)

if __name__ == '__main__':
   main()