#include <stdio.h>
int main(void)
{
    int x, y;

    puts("二つの整数を入力してください。");
    printf("整数x:");   scanf("%d", &x);
    printf("整数y:");   scanf("%d", &y);

    printf("xの値はyの%d%%です。", 100 * x / y);

    return 0;
}