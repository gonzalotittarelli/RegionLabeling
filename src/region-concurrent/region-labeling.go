package main

import (
	"bufio"
	"fmt"
	"log"
	"sync"
	"os"
	"strconv"
	"strings"
	"time"
)

const P = 810

var image [][]int
var label [][]int
var result = make(chan bool)
var first [P]chan [][]int
var second [P]chan [][]int
var answer [P]chan bool
var cresult [P]chan [][]int

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

func replace(i int, matrix [][]int, value [][]int) [][]int {
	//matrix = append(matrix[:i], matrix[i+1:]...)
	matrix = matrix[:i+copy(matrix[i:], matrix[i+1:])]
	matrix = append(matrix[:i], append(value, matrix[i:]...)...)
	return matrix
}

func worker(w int, length int, n int, wg *sync.WaitGroup) {
	stripSize := length / P
	localimage := make([][]int, 2) // local values plus edges
	locallabel := make([][]int, 2) // from neighbors
	for i := range localimage {
		localimage[i] = make([]int, n)
		locallabel[i] = make([]int, n)
	}
	change := true
	limage := make([][]int, len(image[(w*stripSize):((w*stripSize)+stripSize)]))
	copy(limage, image[(w*stripSize):((w*stripSize)+stripSize)])
	llabel := make([][]int, len(label[(w*stripSize):((w*stripSize)+stripSize)]))
	copy(llabel, label[(w*stripSize):((w*stripSize)+stripSize)])
	localimage = append(localimage[:1], append(limage, localimage[1:]...)...)
	locallabel = append(locallabel[:1], append(llabel, locallabel[1:]...)...)
	// exchange edges of image with neighbors
	if w != 0 {
		f := make([][]int, len(localimage[1:2]))
		copy(f, localimage[1:2])
		first[w-1] <- f
	}
	if w != P-1 {
		s := make([][]int, len(localimage[stripSize : stripSize+1]))
		copy(s, localimage[stripSize : stripSize+1])
		second[w+1] <- s
	}
	if w != P-1 {
		var belowimage [][]int
		belowimage = <-first[w]		
		localimage = replace(stripSize+1, localimage, belowimage)
	}
	if w != 0 {
		var aboveimage [][]int
		aboveimage = <-second[w]
		localimage = replace(0, localimage, aboveimage)
	}
	for change {
		change = false
		if w != 0 {
			f := make([][]int, len(locallabel[1:2]))
			copy(f, locallabel[1:2])
			first[w-1] <- f
		}
		if w != P-1 {
			s := make([][]int, len(locallabel[stripSize : stripSize+1]))
			copy(s, locallabel[stripSize : stripSize+1])
			second[w+1] <- s
		}
		if w != P-1 {
			var belowimage [][]int
			belowimage = <-first[w]
			locallabel = replace(stripSize+1, locallabel, belowimage)
		}
		if w != 0 {
			var aboveimage [][]int
			aboveimage = <-second[w]
			locallabel = replace(0, locallabel, aboveimage)
		}
		for i := 1; i <= stripSize; i++ {
			for j := 0; j < n; j++ {
				oldlabel := locallabel[i][j]
				if localimage[i][j] == 1 {
					locallabel[i][j] = maxNeighbours(locallabel, i, j)
				}
				if locallabel[i][j] != oldlabel {
					change = true
				}
			}
		}
		result <- change
		change = <-answer[w]
	}
	cresult[w] <- locallabel[1:len(locallabel)-1]
	wg.Done()
}

func coordinator() {
	chg, change := true, true
	for change {
		change = false
		for i := 0; i < P; i++ {
			chg = <-result
			change = (change || chg)
		}
		for i := 0; i < P; i++ {
			answer[i] <- change
		}
	}
	for i := 0; i < P; i++ {
		<-cresult[i]
		/*llabel := <-cresult[i]
		printMatrix(llabel)*/
	}
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
	image = make([][]int, 0)
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
	label = make([][]int, 0)
	n := len(image[0])
	for i, row := range image {
		tmp := make([]int, 0)
		for j := range row {
			value := 0
			if image[i][j] == 1 {
				value = i*n + j
			}
			tmp = append(tmp, value)
		}
		label = append(label, tmp)
	}

	for i := range answer {
		answer[i] = make(chan bool)
		first[i] = make(chan [][]int, 200)
		second[i] = make(chan [][]int, 200)
		cresult[i] = make(chan [][]int)
	}
	start := time.Now()
	var wg sync.WaitGroup
	for i := 0; i < P; i++ {
		wg.Add(1)
		go worker(i, len(image), len(image[0]), &wg)
	}
	coordinator()
	wg.Wait()
	log.Printf("--- %s seconds ---", (time.Since(start)))
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}	
}
