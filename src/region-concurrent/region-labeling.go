package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)
const P = 5
var image [][]int
var label [][]int
var result = make(chan bool)
var answer [P] chan bool


// Return the maximum value in an integer array.
func max(arr []int) int {
	max_num := arr[0]
	for _, elem := range arr {
		if max_num < elem {
			max_num = elem
		}
	}
	return max_num
}

// Return the maximun neighbour in an matrix at the position i, j
func maxNeighbours(M [][]int, i int, j int) int {
	neighbour := make([]int, 0)
	if i > 0 {
		neighbour = append(neighbour, M[i-1][j])
	}
	if i < (len(M) - 1) {
		neighbour = append(neighbour, M[i+1][j])
	}
	if j > 0 {
		neighbour = append(neighbour, M[i][j-1])
	}
	if j < len(M[i])-1 {
		neighbour = append(neighbour, M[i][j+1])
	}
	return max(neighbour)
}

func worker(i, length int){
	stripSize := length / P
	

}

func coordinator() {
	chg, change := true, true
	for change {
		change = false
		for i:=0; i<= P; i++{
			chg = <- result
			change = (change || chg)
		}
		for i:=0; i< P; i++{
			answer[i] <- change
		}
	}
}

func regionLabeling(M [][]int) [][]int {
	start := time.Now()
	R := make([][]int, 0)
	n := len(M[0])
	for i, row := range M {
		tmp := make([]int, 0)
		for j := range row {
			value := 0
			if M[i][j] == 1 {
				value = i*n + j
			}
			tmp = append(tmp, value)
		}
		R = append(R, tmp)
	}
	for {
		change := false
		for i, row := range M {
			for j := range row {
				oldlabel := R[i][j]
				if M[i][j] == 1 {
					R[i][j] = maxNeighbours(R, i, j)
				}
				if R[i][j] != oldlabel {
					change = true
				}
			}
		}
		if !change {
			break
		}
	}
	log.Printf("--- %s seconds ---", (time.Since(start)))
	return R
}

func printMatrix(M [][]int) {
	for i, row := range M {
		for j := range row {
			if M[i][j] > 0 {
				fmt.Print("*")
			} else {
				fmt.Print(0)
			}
		}
		fmt.Println()
	}
}

func main() {
	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	image := make([][]int, 0)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		tmp := make([]int, 0)
		line := strings.Split(string(scanner.Text()), "")
		for i := range line {
			result := line[i]
			letter, err := strconv.Atoi(result)
			if err != nil {
				log.Fatal(err)
			}
			tmp = append(tmp, letter)
		}
		image = append(image, tmp)
	}
	
	for i := range answer {
		answer[i] = make(chan bool)
	}

	go func() {
		for i:=0; i< P; i++{
			worker(i, len(image))
		} 
	}();

	coordinator()

	//regionLabeling(matrix)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
