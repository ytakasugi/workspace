#include <stdio.h>
int main(void)
{
    double a;

    puts("実数を入力してください。");
    printf("実数a:");
    scanf("%lf", &a);

    printf("貴方は%fを入力しましたね。\n", a);

    return 0;
}