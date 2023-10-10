// https://www.hackerrank.com/challenges/cavity-map/problem
// Cavity Map

package main

import (
    "bufio"
    "fmt"
    "io"
    "os"
    "strconv"
    "strings"
)

/*
 * Complete the 'cavityMap' function below.
 *
 * The function is expected to return a STRING_ARRAY.
 * The function accepts STRING_ARRAY grid as parameter.
 */

func cavityMap(grid []string) []string {
  for i := range grid {
    for j := range grid[i] {
      if (i != 0 && 
          j != 0 && 
          i != (len(grid) - 1) && 
          j != (len(grid) - 1) &&
          int(grid[i][j-1]) < int(grid[i][j]) &&
          int(grid[i-1][j]) < int(grid[i][j]) &&
          int(grid[i+1][j]) < int(grid[i][j]) &&
          int(grid[i][j+1]) < int(grid[i][j])) {
        bytearr := []byte(grid[i])
        bytearr[j] = 'X'
        grid[i] = string(bytearr)
      }
    }
  }
  
  return grid
}

func main() {
    reader := bufio.NewReaderSize(os.Stdin, 16 * 1024 * 1024)
    stdout, err := os.Create(os.Getenv("OUTPUT_PATH"))
    checkError(err)
    defer stdout.Close()
    writer := bufio.NewWriterSize(stdout, 16 * 1024 * 1024)
    nTemp, err := strconv.ParseInt(strings.TrimSpace(readLine(reader)), 10, 64)
    checkError(err)
    n := int32(nTemp)
    var grid []string
    for i := 0; i < int(n); i++ {
        gridItem := readLine(reader)
        grid = append(grid, gridItem)
    }
    result := cavityMap(grid)
    for i, resultItem := range result {
        fmt.Fprintf(writer, "%s", resultItem)
        if i != len(result) - 1 {
            fmt.Fprintf(writer, "\n")
        }
    }
    fmt.Fprintf(writer, "\n")
    writer.Flush()
}

func readLine(reader *bufio.Reader) string {
    str, _, err := reader.ReadLine()
    if err == io.EOF {
        return ""
    }
    return strings.TrimRight(string(str), "\r\n")
}

func checkError(err error) {
    if err != nil {
        panic(err)
    }
}
