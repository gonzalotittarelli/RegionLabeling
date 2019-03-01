package main

import (
	"bufio"
    "fmt"
    "log"
    "os"
    "strings"
    "strconv"
)

func regionLabeling(matrix [][]int) {
	for i := range matrix {
		fmt.Println(matrix[i])
	}
}

func main() {
	file, err := os.Open(os.Args[1])
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()
    matrix := make([][]int,0)
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        tmp := make([]int, 0)
        line := strings.Split(string(scanner.Text()), "")
        for i := range(line) {
            result := line[i]
            letter, err := strconv.Atoi(result)
            if err != nil {
                log.Fatal(err)
            }
            tmp = append(tmp, letter)
        }
        matrix = append(matrix, tmp)
    }    
    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }
}