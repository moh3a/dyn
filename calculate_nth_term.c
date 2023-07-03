// https://www.hackerrank.com/challenges/recursion-in-c/problem?isFullScreen=true

#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

/*
A function that calls itself is known as a recursive function. The C programming language supports recursion. But while using recursion, one needs to be careful to define an exit condition from the function, otherwise it will go into an infinite loop.

To prevent infinite recursion, if...else  statement (or similar approach) can be used where one branch makes the recursive call and other doesn't.
*/

int find_nth_term(int n, int a, int b, int c)
{
    // just loop --- no recursion
    // int terms[n];
    // terms[0] = a;
    // terms[1] = b;
    // terms[2] = c;
    // for (int i = 3; i < n; i++) {
    //   terms[i] = terms[i-1] + terms[i-2] + terms[i-3];
    // }
    // return terms[n - 1];

    // with recursion
    if (n == 1)
    {
        return a;
    }
    else if (n == 2)
    {
        return b;
    }
    else if (n == 3)
    {
        return c;
    }
    else
    {
        return find_nth_term(n - 1, a, b, c) + find_nth_term(n - 2, a, b, c) + find_nth_term(n - 3, a, b, c);
    }
}

int main()
{
    int n, a, b, c;

    scanf("%d %d %d %d", &n, &a, &b, &c);
    int ans = find_nth_term(n, a, b, c);

    printf("%d", ans);
    return 0;
}
