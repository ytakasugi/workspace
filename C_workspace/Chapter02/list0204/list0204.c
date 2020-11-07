#include <stdio.h>
int main(void)
{
    int n;

    printf("整数を入力してください:");
    scanf("%d", &n);

    printf("符号を反転した値は%dです。　\n", -n);

    return 0;
    
}