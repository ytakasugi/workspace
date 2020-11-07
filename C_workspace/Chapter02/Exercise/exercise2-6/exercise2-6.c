#include <stdio.h>
int main(void)
{
    int h;
    double w;

    puts("身長を入力してください。");
    printf("身長:");    scanf("%d", &h);

    w = (double)(h - 100) * 0.9;

    printf("あなたの標準体重は%4.1fです。 \n", w);

    return 0;
}