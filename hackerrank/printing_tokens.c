// https://www.hackerrank.com/challenges/printing-tokens-/problem

#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

int main()
{
    printf("Given a sentence, print each word of the sentence in a new line. Provide your sentence:\n");

    char *s;
    s = malloc(1024 * sizeof(char));
    scanf("%[^\n]", s);

    s = realloc(s, strlen(s) + 1);

    printf("\nOutput:\n");

    for (int i = 0; i < strlen(s); i++)
    {
        if (s[i] == ' ')
        {
            printf("\n");
        }
        else
        {
            printf("%c", s[i]);
        }
    }

    return 0;
}
