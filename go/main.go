package main

import "os"
import "fmt"
import "bufio"

func main() {
    file, err := os.Open("../measurements.data")
    if err != nil {
		return
    }
    defer file.Close()

    lines := []string{}

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
    	fmt.Println(scanner.Text())
    	lines = append(lines, scanner.Text())
    }

}
