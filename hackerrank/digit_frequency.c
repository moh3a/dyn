// https://www.hackerrank.com/challenges/frequency-of-digits-1/problem

#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>
#include <stdbool.h>

int main()
{

    char *num;
    num = malloc(1000 * sizeof(char));
    printf("Given a string, , consisting of alphabets and digits, find the frequency of each digit in the given string.\n");
    printf("1 <= len(input) <= 1000\n");
    printf("All the elements of num are made of english alphabets and digits.\n");
    printf("Enter a string:\n");
    scanf("%s", num);
    printf("\n");

    int num_len = (int)strlen(num);
    num = realloc(num, strlen(num) + 1);

    int frequency[10] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

    for (int i = 0; i < num_len; i++)
    {
        char num_char = num[i];
        int num_int = num_char - '0';
        if (num_int >= 0 && num_int < 10)
        {
            frequency[num_int] += 1;
        }
    }

    printf("\nHere we print ten space-separated integers in a single line denoting the frequency of each digit from 0 to 9.\n");
    for (int i = 0; i < sizeof(frequency) / sizeof(int); i++)
    {
        printf("%d ", frequency[i]);
    }

    free(num);
    return 0;
}
