#include <stdio.h>
int main(void)
{
    int n;

    printf("整数を入力してください。:");
    scanf("%d", &n);

    if(n % 5)
        puts("その数は5で割り切れません。");

    return 0;
}