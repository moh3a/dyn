// https://www.hackerrank.com/challenges/organizing-containers-of-balls/problem
// Organizing Containers of Balls

package main

import (
    "bufio"
    "fmt"
    "io"
    "os"
    "strconv"
    "strings"
    "sort"
)

/*
 * Complete the 'organizingContainers' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts 2D_INTEGER_ARRAY container as parameter.
 */

func organizingContainers(container [][]int32) string {
  containerSlice := make([]int, len(container))
  ballTypeSlice := make([]int, len(container))
  for i := range container {
    for j := range container {
      containerSlice[i] += int(container[i][j])
      ballTypeSlice[i] += int(container[j][i])
    }
  }
  sort.Ints(containerSlice)
  sort.Ints(ballTypeSlice)
  result := "Possible"
  for i := range containerSlice {
    if containerSlice[i] != ballTypeSlice[i] {
      result = "Impossible"
      break
    }
  }
  return result
}

func main() {
    reader := bufio.NewReaderSize(os.Stdin, 16 * 1024 * 1024)
    stdout, err := os.Create(os.Getenv("OUTPUT_PATH"))
    checkError(err)
    defer stdout.Close()
    writer := bufio.NewWriterSize(stdout, 16 * 1024 * 1024)
    qTemp, err := strconv.ParseInt(strings.TrimSpace(readLine(reader)), 10, 64)
    checkError(err)
    q := int32(qTemp)
    for qItr := 0; qItr < int(q); qItr++ {
        nTemp, err := strconv.ParseInt(strings.TrimSpace(readLine(reader)), 10, 64)
        checkError(err)
        n := int32(nTemp)
        var container [][]int32
        for i := 0; i < int(n); i++ {
            containerRowTemp := strings.Split(strings.TrimRight(readLine(reader)," \t\r\n"), " ")
            var containerRow []int32
            for _, containerRowItem := range containerRowTemp {
                containerItemTemp, err := strconv.ParseInt(containerRowItem, 10, 64)
                checkError(err)
                containerItem := int32(containerItemTemp)
                containerRow = append(containerRow, containerItem)
            }
            if len(containerRow) != int(n) {
                panic("Bad input")
            }
            container = append(container, containerRow)
        }
        result := organizingContainers(container)
        fmt.Fprintf(writer, "%s\n", result)
    }
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