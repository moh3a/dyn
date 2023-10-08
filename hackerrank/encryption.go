// https://www.hackerrank.com/challenges/encryption/problem
// Encryption

package main

import (
    "bufio"
    "fmt"
    "io"
    "os"
    "strings"
    "math"
)

/*
 * Complete the 'encryption' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

func encryption(s string) string {
  length := len(s)
  rows := int(math.Floor(math.Sqrt(float64(length))))
  columns := int(math.Ceil(math.Sqrt(float64(length))))
  var result []string
  for i := range make([]int, columns) {
    temp := make([]string, rows)
    j := 0
    for i + j < length {
      temp = append(temp, string(s[i+j]))
      j += columns
    }
    result = append(result, strings.Join(temp, ""))
  }
  return strings.Join(result, " ")
}

func main() {
    reader := bufio.NewReaderSize(os.Stdin, 16 * 1024 * 1024)
    stdout, err := os.Create(os.Getenv("OUTPUT_PATH"))
    checkError(err)
    defer stdout.Close()
    writer := bufio.NewWriterSize(stdout, 16 * 1024 * 1024)
    s := readLine(reader)
    result := encryption(s)
    fmt.Fprintf(writer, "%s\n", result)
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
