package main

import (
	"fmt"
	//"log"
	"time"
)

const P = 5

var image [][]int
var label [][]int
var result = make(chan bool)
var first [P]chan [][]int
var second [P]chan [][]int
var answer [P]chan bool

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
	matrix = append(matrix[:i], matrix[i+1:]...)
	matrix = append(matrix[:i], append(value, matrix[i:]...)...)
	return matrix
}

func worker(i int, length int, n int) {	
	stripSize := length / P
	localimage := make([][]int, 2) // local values plus edges
	locallabel := make([][]int, 2) // from neighbors
	for i := range localimage {
		localimage[i] = make([]int, n)
		locallabel[i] = make([]int, n)
	}
	change := true
	localimage = append(localimage[:1], append(image[(i*stripSize):((i*stripSize)+stripSize)], localimage[1:]...)...)
	locallabel = append(locallabel[:1], append(label[(i*stripSize):((i*stripSize)+stripSize)], locallabel[1:]...)...)
	// exchange edges of image with neighbors
	if i != 0 {
		first[i-1] <- localimage[1:2]
	}	
	if i != P-1 {
		second[i+1] <- localimage[stripSize : stripSize+1]
	}
	if i != P-1 {
		var belowimage [][]int
		belowimage = <-first[i]
		//localimage = append(localimage[:stripSize], append(belowimage, localimage[stripSize:]...)...)
		localimage = replace(stripSize, localimage, belowimage)
	}
	if i != 0 {
		var aboveimage [][]int
		aboveimage = <-second[i]
		//localimage = append(localimage[:0], append(aboveimage, localimage[0:]...)...)
		localimage = replace(0, localimage, aboveimage)
	}	
	for change {
		change = false
		if i != 0 {
			first[i-1] <- locallabel[1:2]
		}
		if i != P-1 {
			second[i+1] <- locallabel[stripSize : stripSize+1]
		}
		if i != P-1 {
			var belowimage [][]int
			belowimage = <-first[i]
			//locallabel = append(locallabel[:stripSize], append(belowimage, locallabel[stripSize:]...)...)
			locallabel = replace(stripSize, locallabel, belowimage)
		}
		if i != 0 {
			var aboveimage [][]int
			aboveimage = <-second[i]
			//locallabel = append(locallabel[:0], append(aboveimage, locallabel[0:]...)...)
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
		change = <-answer[i]
	}
	fmt.Println(locallabel)
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
}

func printMatrix(M [][]int) {
	for i, row := range M {
		for j := range row {
			if M[i][j] > 0 {
				fmt.Print(M[i][j])
			} else {
				fmt.Print(0)
			}
		}
		fmt.Println()
	}
}

func main() {
	/*file, err := os.Open(os.Args[1])
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
	}*/
	image = [][]int{
		{0, 0, 0, 0},
		{0, 0, 0, 0},
		{0, 1, 1, 0},
		{0, 1, 1, 0},
		{0, 0, 1, 0},
		{0, 0, 1, 1},
		{0, 0, 1, 1},
		{0, 0, 1, 0},
		{1, 1, 1, 0},
		{1, 1, 0, 0},
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
		answer[i] = make(chan bool, 100)
		first[i] = make(chan [][]int, 100)
		second[i] = make(chan [][]int, 100)
	}

	for i := 0; i < P; i++ {
		go worker(i, len(image), len(image[0]))
	}
	
	coordinator()	
	time.Sleep(3 * time.Second)
	/*if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}*/
}
