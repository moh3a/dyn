#include <stdio.h>
#include <stdlib.h>

int main()
{
    printf("Given an array, of size 'n', reverse it.");
    printf("First provide 'n' length of the array:\n");

    int num, *arr, i;
    scanf("%d", &num);
    arr = (int *)malloc(num * sizeof(int));

    printf("Now provide 'n' space separated integers:\n");
    for (i = 0; i < num; i++)
    {
        scanf("%d", arr + i);
    }

    int *new_arr = (int *)malloc(num * sizeof(int));
    for (i = 0; i < num; i++)
        new_arr[i] = arr[num - 1 - i];

    free(arr);

    printf("\nOutput:\n");
    for (i = 0; i < num; i++)
        printf("%d ", *(new_arr + i));
    return 0;
}
