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

func find_neighbours(M [][]int, R [][]int, i int, j int, queue *[][]int, current *int) {
	if i > 0 {
		if M[i-1][j] == 1 && R[i-1][j] == 0 {
			R[i-1][j] = *current
			*queue = append(*queue, make([]int, i-1, j))
		}
	}
	if i < len(M)-1 {
		if M[i+1][j] == 1 && R[i+1][j] == 0 {
			R[i+1][j] = *current
			*queue = append(*queue, make([]int, i+1, j))
		}
	}
	if j > 0 {
		if M[i][j-1] == 1 && R[i][j-1] == 0 {
			R[i][j-1] = *current
			*queue = append(*queue, make([]int, i, j-1))
		}
	}
	if j < len(M[i])-1 {
		if M[i][j+1] == 1 && R[i][j+1] == 0 {
			R[i][j+1] = *current
			*queue = append(*queue, make([]int, i, j+1))
		}
	}
}

func regionLabeling(M [][]int) [][]int {
	start := time.Now()
	R := make([][]int, 0)
	n := len(M[0])
	current := 1
	queue := make([][]int, 0)
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
	for i, row := range M {
		for j := range row {
			if M[i][j] == 1 && R[i][j] == 0 {
				R[i][j] = current
				queue = append(queue, make([]int, i, j))
				for len(queue) > 0 {
					element := queue[0]
					queue = queue[1:]
					find_neighbours(M, R, element[0], element[1], &queue, &current)
				}
				current++
			}
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
	matrix := make([][]int, 0)
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
		matrix = append(matrix, tmp)
	}
	label := regionLabeling(matrix)
	printMatrix(label)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
