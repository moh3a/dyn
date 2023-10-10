// https://www.hackerrank.com/challenges/acm-icpc-team/problem
// ACM ICPC Team

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
There are a number of people who will be attending ACM-ICPC World Finals. Each of them may be well versed in a number of topics. Given a list of topics known by each attendee, presented as binary strings, determine the maximum number of topics a 2-person team can know. Each subject has a column in the binary string, and a '1' means the subject is known while '0' means it is not. Also determine the number of teams that know the maximum number of topics. Return an integer array with two elements. The first is the maximum number of topics known, and the second is the number of teams that know that number of topics.
*/

/*
 * Complete the 'acmTeam' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts STRING_ARRAY topic as parameter.
 */

func acmTeam(topic []string) []int32 {
	var max_num_subjects int32 = 0
	var teams_count int32 = 0
	for i := 0; i < len(topic)-1; i++ {
		for j := 1; j < len(topic); j++ {
			var current_subject_count int32 = 0
			for k := 0; k < len(topic[i]); k++ {
				if j > i && (topic[i][k] == '1' || topic[j][k] == '1') {
					current_subject_count++
				}
			}
			if current_subject_count > max_num_subjects {
				max_num_subjects = current_subject_count
				teams_count = 1
			} else if current_subject_count == max_num_subjects {
				teams_count++
			}
		}
	}
	var result = []int32{max_num_subjects, teams_count}
	return result
}

func main() {
	reader := bufio.NewReaderSize(os.Stdin, 16*1024*1024)
	stdout, err := os.Create(os.Getenv("OUTPUT_PATH"))
	checkError(err)
	defer stdout.Close()
	writer := bufio.NewWriterSize(stdout, 16*1024*1024)
	firstMultipleInput := strings.Split(strings.TrimSpace(readLine(reader)), " ")
	nTemp, err := strconv.ParseInt(firstMultipleInput[0], 10, 64)
	checkError(err)
	n := int32(nTemp)
	// mTemp, err := strconv.ParseInt(firstMultipleInput[1], 10, 64)
	// checkError(err)
	// m := int32(mTemp)
	var topic []string
	for i := 0; i < int(n); i++ {
		topicItem := readLine(reader)
		topic = append(topic, topicItem)
	}
	result := acmTeam(topic)
	for i, resultItem := range result {
		fmt.Fprintf(writer, "%d", resultItem)
		if i != len(result)-1 {
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
